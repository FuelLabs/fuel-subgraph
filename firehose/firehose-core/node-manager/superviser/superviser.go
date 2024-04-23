// Copyright 2019 dfuse Platform Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package superviser

import (
	"fmt"
	"strings"
	"sync"
	"time"

	"github.com/ShinyTrinkets/overseer"
	"github.com/streamingfast/bstream"
	nodeManager "github.com/streamingfast/firehose-core/node-manager"
	logplugin "github.com/streamingfast/firehose-core/node-manager/log_plugin"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
)

// mindreaderPlugin can be used to check if `logplugin.LogPlugin` is actually a mindreader one.
// This is not in `mindreader` package to not introduce a cycle dependencies
type mindreaderPlugin interface {
	logplugin.LogPlugin

	LastSeenBlock() bstream.BlockRef
}

type Superviser struct {
	*shutter.Shutter
	Binary    string
	Arguments []string
	// Env represents the environment variables the command will run with, the `nil`
	// is handled differently than the `[]string{}` empty case. In the `nil` case,
	// the process inherits from the parent process. In the empty case, it starts
	// without any variables set.
	Env    []string
	Logger *zap.Logger

	cmd     *overseer.Cmd
	cmdLock sync.Mutex

	logPlugins     []logplugin.LogPlugin
	logPluginsLock sync.RWMutex

	enableDeepMind bool
}

func New(logger *zap.Logger, binary string, arguments []string) *Superviser {
	s := &Superviser{
		Shutter:   shutter.New(),
		Binary:    binary,
		Arguments: arguments,
		Logger:    logger,
	}

	s.Shutter.OnTerminating(func(_ error) {
		s.Logger.Info("superviser is terminating")

		if err := s.Stop(); err != nil {
			s.Logger.Error("failed to stop supervised node process", zap.Error(err))
		}

		s.Logger.Info("shutting down plugins", zap.Int("last_exit_code", s.LastExitCode()))
		s.endLogPlugins()
	})

	return s
}

func (s *Superviser) RegisterLogPlugin(plugin logplugin.LogPlugin) {
	s.logPluginsLock.Lock()
	defer s.logPluginsLock.Unlock()

	s.logPlugins = append(s.logPlugins, plugin)
	if shut, ok := plugin.(logplugin.Shutter); ok {
		s.Logger.Info("adding superviser shutdown to plugins", zap.String("plugin_name", plugin.Name()))
		shut.OnTerminating(func(err error) {
			if !s.IsTerminating() {
				s.Logger.Info("superviser shutting down because of a plugin", zap.String("plugin_name", plugin.Name()))
				go s.Shutdown(err)
			}
		})
	}

	s.Logger.Info("registered log plugin", zap.Int("plugin count", len(s.logPlugins)))
}

func (s *Superviser) GetLogPlugins() []logplugin.LogPlugin {
	s.logPluginsLock.RLock()
	defer s.logPluginsLock.RUnlock()

	return s.logPlugins
}

func (s *Superviser) setDeepMindDebug(enabled bool) {
	s.Logger.Info("setting deep mind debug mode", zap.Bool("enabled", enabled))
	for _, logPlugin := range s.logPlugins {
		if v, ok := logPlugin.(nodeManager.DeepMindDebuggable); ok {
			v.DebugDeepMind(enabled)
		}
	}
}

func (s *Superviser) Stopped() <-chan struct{} {
	if s.cmd != nil {
		return s.cmd.Done()
	}
	return nil
}

func (s *Superviser) LastExitCode() int {
	if s.cmd != nil {
		return s.cmd.Status().Exit
	}
	return 0
}

func (s *Superviser) LastLogLines() []string {
	if s.hasToConsolePlugin() {
		// There is no point in showing the last log lines when the user already saw it through the to console log plugin
		return nil
	}

	for _, plugin := range s.logPlugins {
		if v, ok := plugin.(*logplugin.KeepLastLinesLogPlugin); ok {
			return v.LastLines()
		}
	}

	return nil
}

func (s *Superviser) LastSeenBlockNum() uint64 {
	for _, plugin := range s.GetLogPlugins() {
		if v, ok := plugin.(mindreaderPlugin); ok {
			return v.LastSeenBlock().Num()
		}
	}
	return 0
}

func (s *Superviser) Start(options ...nodeManager.StartOption) error {
	for _, opt := range options {
		if opt == nodeManager.EnableDebugDeepmindOption {
			s.setDeepMindDebug(true)
		}
		if opt == nodeManager.DisableDebugDeepmindOption {
			s.setDeepMindDebug(false)
		}
	}

	for _, plugin := range s.logPlugins {
		plugin.Launch()
	}

	s.cmdLock.Lock()
	defer s.cmdLock.Unlock()

	if s.cmd != nil {
		if s.cmd.State == overseer.STARTING || s.cmd.State == overseer.RUNNING {
			s.Logger.Info("underlying process already running, nothing to do")
			return nil
		}

		if s.cmd.State == overseer.STOPPING {
			s.Logger.Info("underlying process is currently stopping, waiting for it to finish")
			<-s.cmd.Done()
		}
	}

	s.Logger.Info("creating new command instance and launch read loop", zap.String("binary", s.Binary), zap.Strings("arguments", s.Arguments))
	var args []interface{}
	for _, a := range s.Arguments {
		args = append(args, a)
	}

	s.cmd = overseer.NewCmd(s.Binary, s.Arguments, overseer.Options{Streaming: true, Env: s.Env})

	go s.start(s.cmd)

	return nil
}

func (s *Superviser) Stop() error {
	s.cmdLock.Lock()
	defer s.cmdLock.Unlock()

	s.Logger.Info("supervisor received a stop request, terminating supervised node process")

	if !s.isRunning() {
		s.Logger.Info("underlying process is not running, nothing to do")
		return nil
	}

	if s.cmd.State == overseer.STARTING || s.cmd.State == overseer.RUNNING {
		s.Logger.Info("stopping underlying process")
		err := s.cmd.Stop()
		if err != nil {
			s.Logger.Error("failed to stop overseer cmd", zap.Error(err))
			return err
		}
	}

	// Blocks until command finished completely
	s.Logger.Debug("blocking until command actually ends")

nodeProcessDone:
	for {
		select {
		case <-s.cmd.Done():
			break nodeProcessDone
		case <-time.After(500 * time.Millisecond):
			s.Logger.Debug("still blocking until command actually ends")
		}
	}

	s.Logger.Info("supervised process has been terminated")

	s.Logger.Info("waiting for stdout and stderr to be drained", s.getProcessOutputStatsLogFields()...)
	for {
		if s.isBufferEmpty() {
			break
		}

		s.Logger.Debug("draining stdout and stderr", s.getProcessOutputStatsLogFields()...)
		time.Sleep(500 * time.Millisecond)
	}

	s.Logger.Info("stdout and stderr are now drained")

	// Must be after `for { ... }` as `s.cmd` is used within the loop and also before it via call to `getProcessOutputStatsLogFields`
	s.cmd = nil

	return nil
}

func (s *Superviser) getProcessOutputStats() (stdoutLineCount, stderrLineCount int) {
	if s.cmd != nil {
		return len(s.cmd.Stdout), len(s.cmd.Stderr)
	}

	return
}

func (s *Superviser) getProcessOutputStatsLogFields() []zap.Field {
	stdoutLineCount, stderrLineCount := s.getProcessOutputStats()

	return []zap.Field{zap.Int("stdout_len", stdoutLineCount), zap.Int("stderr_len", stderrLineCount)}
}

func (s *Superviser) IsRunning() bool {
	s.cmdLock.Lock()
	defer s.cmdLock.Unlock()

	return s.isRunning()
}

// This one assuming the lock is properly held already
func (s *Superviser) isRunning() bool {
	if s.cmd == nil {
		return false
	}
	return s.cmd.State == overseer.STARTING || s.cmd.State == overseer.RUNNING || s.cmd.State == overseer.STOPPING
}

func (s *Superviser) isBufferEmpty() bool {
	if s.cmd == nil {
		return true
	}
	return len(s.cmd.Stdout) == 0 && len(s.cmd.Stderr) == 0
}

func (s *Superviser) start(cmd *overseer.Cmd) {
	statusChan := cmd.Start()

	processTerminated := false
	for {
		select {
		case status := <-statusChan:
			processTerminated = true
			if status.Exit == 0 {
				s.Logger.Info("command terminated with zero status", s.getProcessOutputStatsLogFields()...)
			} else {
				s.Logger.Error(fmt.Sprintf("command terminated with non-zero status, last log lines:\n%s\n", formatLogLines(s.LastLogLines())), overseerStatusLogFields(status)...)
			}

		case line := <-cmd.Stdout:
			s.processLogLine(line)
		case line := <-cmd.Stderr:
			s.processLogLine(line)
		}

		if processTerminated {
			s.Logger.Debug("command terminated but continue read loop to fully consume stdout/sdterr line channels", zap.Bool("buffer_empty", s.isBufferEmpty()))
			if s.isBufferEmpty() {
				return
			}
		}
	}
}

func overseerStatusLogFields(status overseer.Status) []zap.Field {
	fields := []zap.Field{
		zap.String("command", status.Cmd),
		zap.Int("exit_code", status.Exit),
	}

	if status.Error != nil {
		fields = append(fields, zap.String("error", status.Error.Error()))
	}

	if status.PID != 0 {
		fields = append(fields, zap.Int("pid", status.PID))
	}

	if status.Runtime > 0 {
		fields = append(fields, zap.Duration("runtime", time.Duration(status.Runtime*float64(time.Second))))
	}

	if status.StartTs > 0 {
		fields = append(fields, zap.Time("started_at", time.Unix(0, status.StartTs)))
	}

	if status.StopTs > 0 {
		fields = append(fields, zap.Time("stopped_at", time.Unix(0, status.StopTs)))
	}

	return fields
}

func formatLogLines(lines []string) string {
	if len(lines) == 0 {
		return "<None>"
	}

	formattedLines := make([]string, len(lines))
	for i, line := range lines {
		formattedLines[i] = "  " + line
	}

	return strings.Join(formattedLines, "\n")
}

func (s *Superviser) endLogPlugins() {
	s.logPluginsLock.Lock()
	defer s.logPluginsLock.Unlock()

	for _, plugin := range s.logPlugins {
		s.Logger.Info("stopping plugin", zap.String("plugin_name", plugin.Name()))
		plugin.Stop()
	}
	s.Logger.Info("all plugins closed")
}

func (s *Superviser) processLogLine(line string) {
	s.logPluginsLock.Lock()
	defer s.logPluginsLock.Unlock()

	for _, plugin := range s.logPlugins {
		plugin.LogLine(line)
	}
}

func (s *Superviser) hasToConsolePlugin() bool {
	for _, plugin := range s.logPlugins {
		if _, ok := plugin.(*logplugin.ToConsoleLogPlugin); ok {
			return true
		}
	}

	return false
}
