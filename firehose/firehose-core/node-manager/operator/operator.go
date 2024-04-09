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

package operator

import (
	"fmt"
	"net/http"
	"os"
	"strings"
	"sync"
	"time"

	"github.com/streamingfast/derr"
	nodeManager "github.com/streamingfast/firehose-core/node-manager"
	"github.com/streamingfast/shutter"
	"go.uber.org/atomic"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

type Operator struct {
	*shutter.Shutter
	options          *Options
	lastStartCommand time.Time

	backupModules   map[string]BackupModule
	backupSchedules []*BackupSchedule

	commandChan chan *Command
	httpServer  *http.Server

	Superviser     nodeManager.ChainSuperviser
	chainReadiness nodeManager.Readiness

	aboutToStop *atomic.Bool
	zlogger     *zap.Logger
}

type Options struct {
	Bootstrapper Bootstrapper

	EnableSupervisorMonitoring bool

	// Delay before sending Stop() to superviser, during which we return NotReady
	ShutdownDelay time.Duration
}

type Command struct {
	cmd      string
	params   map[string]string
	returnch chan error
	closer   sync.Once
	logger   *zap.Logger
}

func (c *Command) MarshalLogObject(encoder zapcore.ObjectEncoder) error {
	encoder.AddString("name", c.cmd)
	encoder.AddReflected("params", c.params)
	return nil
}

func New(zlogger *zap.Logger, chainSuperviser nodeManager.ChainSuperviser, chainReadiness nodeManager.Readiness, options *Options) (*Operator, error) {
	zlogger.Info("creating operator", zap.Reflect("options", options))

	o := &Operator{
		Shutter:        shutter.New(),
		chainReadiness: chainReadiness,
		commandChan:    make(chan *Command, 10),
		options:        options,
		Superviser:     chainSuperviser,
		aboutToStop:    atomic.NewBool(false),
		zlogger:        zlogger,
	}

	chainSuperviser.OnTerminated(func(err error) {
		if !o.IsTerminating() {
			zlogger.Info("chain superviser is shutting down operator")
			o.Shutdown(err)
		}
	})

	o.OnTerminating(func(err error) {
		//wait for supervisor to terminate, supervisor will wait for plugins to terminate
		if !chainSuperviser.IsTerminating() {
			zlogger.Info("operator is terminating", zap.Error(err))
			chainSuperviser.Shutdown(err)
		}

		zlogger.Info("operator is waiting for superviser to shutdown", zap.Error(err))
		<-o.Superviser.Terminated()
		zlogger.Info("operator done waiting for superviser to shutdown", zap.Error(err))
	})

	return o, nil
}

func (o *Operator) Launch(httpListenAddr string, options ...HTTPOption) error {
	o.zlogger.Info("launching operator HTTP server", zap.String("http_listen_addr", httpListenAddr))
	o.httpServer = o.RunHTTPServer(httpListenAddr, options...)

	// FIXME: too many options for that, maybe use monitoring module like with bootstrapper
	if o.options.EnableSupervisorMonitoring {
		if monitorable, ok := o.Superviser.(nodeManager.MonitorableChainSuperviser); ok {
			go monitorable.Monitor()
		}
	}

	o.LaunchBackupSchedules()

	if o.options.Bootstrapper != nil {
		o.zlogger.Info("operator calling bootstrap function")
		err := o.options.Bootstrapper.Bootstrap()
		if err != nil {
			return fmt.Errorf("unable to bootstrap chain: %w", err)
		}
	}
	o.commandChan <- &Command{cmd: "start", logger: o.zlogger}

	for {
		o.zlogger.Info("operator ready to receive commands")
		select {
		case <-o.Superviser.Stopped(): // the chain stopped outside of a command that was expecting it.
			if o.Superviser.IsTerminating() {
				o.zlogger.Info("superviser terminating, waiting for operator...")
				<-o.Terminating()
				return o.Err()
			}
			// FIXME call a restore handler if passed...
			lastLogLines := o.Superviser.LastLogLines()

			// FIXME: Actually, we should create a custom error type that contains the required data, the catching
			//        code can thus perform the required formatting!
			baseFormat := "instance %q stopped (exit code: %d), shutting down"
			var shutdownErr error
			if len(lastLogLines) > 0 {
				shutdownErr = fmt.Errorf(baseFormat+": last log lines:\n%s", o.Superviser.GetName(), o.Superviser.LastExitCode(), formatLogLines(lastLogLines))
			} else {
				shutdownErr = fmt.Errorf(baseFormat, o.Superviser.GetName(), o.Superviser.LastExitCode())
			}

			o.Shutdown(shutdownErr)
			break

		case cmd := <-o.commandChan:
			if cmd.cmd == "start" { // start 'sub' commands after a restore do NOT come through here
				o.lastStartCommand = time.Now()
			}
			err := o.runCommand(cmd)
			cmd.Return(err)
			if err != nil {
				if err == ErrCleanExit {
					return nil
				}
				return fmt.Errorf("command %v execution failed: %v", cmd.cmd, err)
			}
		}
	}
}

func formatLogLines(lines []string) string {
	formattedLines := make([]string, len(lines))
	for i, line := range lines {
		formattedLines[i] = "  " + line
	}

	return strings.Join(formattedLines, "\n")
}

func (o *Operator) runSubCommand(name string, parentCmd *Command) error {
	return o.runCommand(&Command{cmd: name, returnch: parentCmd.returnch, logger: o.zlogger})
}

func (o *Operator) cleanSuperviserStop() error {
	o.aboutToStop.Store(true)
	defer o.aboutToStop.Store(false)
	if o.options.ShutdownDelay != 0 && !derr.IsShuttingDown() {
		o.zlogger.Info("marked as not_ready, waiting delay before actually stopping for maintenance", zap.Duration("delay", o.options.ShutdownDelay))
		time.Sleep(o.options.ShutdownDelay)
	}

	err := o.Superviser.Stop()
	return err
}

// runCommand does its work, and returns an error for irrecoverable states.
func (o *Operator) runCommand(cmd *Command) error {
	o.zlogger.Info("received operator command", zap.String("command", cmd.cmd), zap.Reflect("params", cmd.params))
	switch cmd.cmd {
	case "maintenance":
		o.zlogger.Info("preparing to stop process")

		if err := o.cleanSuperviserStop(); err != nil {
			return err
		}

		// Careful, we are now "stopped". Every other case can handle that state.
		o.zlogger.Info("successfully put in maintenance")

	case "restore":
		restoreMod, err := selectRestoreModule(o.backupModules, cmd.params["name"])
		if err != nil {
			cmd.Return(err)
			return nil
		}

		o.zlogger.Info("Stopping to restore a backup")
		if restoreMod.RequiresStop() {
			if err := o.cleanSuperviserStop(); err != nil {
				return err
			}
		}

		backupName := "latest"
		if b, ok := cmd.params["backupName"]; ok {
			backupName = b
		}

		if err := restoreMod.Restore(backupName); err != nil {
			return err
		}

		o.zlogger.Info("Restarting after restore")
		if restoreMod.RequiresStop() {
			return o.runSubCommand("start", cmd)
		}
		return nil

	case "backup":
		backupMod, err := selectBackupModule(o.backupModules, cmd.params["name"])
		if err != nil {
			cmd.Return(err)
			return nil
		}

		o.zlogger.Info("Stopping to perform a backup")
		if backupMod.RequiresStop() {
			if err := o.cleanSuperviserStop(); err != nil {
				return err
			}
		}

		backupName, err := backupMod.Backup(uint32(o.Superviser.LastSeenBlockNum()))
		if err != nil {
			return err
		}
		cmd.logger.Info("Completed backup", zap.String("backup_name", backupName))

		o.zlogger.Info("Restarting after backup")
		if backupMod.RequiresStop() {
			return o.runSubCommand("start", cmd)
		}
		return nil

	case "reload":
		o.zlogger.Info("preparing for reload")
		if err := o.cleanSuperviserStop(); err != nil {
			return err
		}

		return o.runSubCommand("start", cmd)

	case "safely_resume_production":
		o.zlogger.Info("preparing for safely resume production")
		producer, ok := o.Superviser.(nodeManager.ProducerChainSuperviser)
		if !ok {
			cmd.Return(fmt.Errorf("the chain superviser does not support producing blocks"))
			return nil
		}

		isProducing, err := producer.IsProducing()
		if err != nil {
			cmd.Return(fmt.Errorf("unable to check if producing: %w", err))
			return nil
		}

		if !isProducing {
			o.zlogger.Info("resuming production of blocks")
			err := producer.ResumeProduction()
			if err != nil {
				cmd.Return(fmt.Errorf("error resuming production of blocks: %w", err))
				return nil
			}

			o.zlogger.Info("successfully resumed producer")

		} else {
			o.zlogger.Info("block production was already running, doing nothing")
		}

		o.zlogger.Info("successfully resumed block production")

	case "safely_pause_production":
		o.zlogger.Info("preparing for safely pause production")
		producer, ok := o.Superviser.(nodeManager.ProducerChainSuperviser)
		if !ok {
			cmd.Return(fmt.Errorf("the chain superviser does not support producing blocks"))
			return nil
		}

		isProducing, err := producer.IsProducing()
		if err != nil {
			cmd.Return(fmt.Errorf("unable to check if producing: %w", err))
			return nil
		}

		if !isProducing {
			o.zlogger.Info("block production is already paused, command is a no-op")
			return nil
		}

		o.zlogger.Info("waiting to pause the producer")
		err = producer.WaitUntilEndOfNextProductionRound(3 * time.Minute)
		if err != nil {
			cmd.Return(fmt.Errorf("timeout waiting for production round: %w", err))
			return nil
		}

		o.zlogger.Info("pausing block production")
		err = producer.PauseProduction()
		if err != nil {
			cmd.Return(fmt.Errorf("unable to pause production correctly: %w", err))
			return nil
		}

		o.zlogger.Info("successfully paused block production")

	case "safely_reload":
		o.zlogger.Info("preparing for safely reload")
		producer, ok := o.Superviser.(nodeManager.ProducerChainSuperviser)
		if ok && producer.IsActiveProducer() {
			o.zlogger.Info("waiting right after production round")
			err := producer.WaitUntilEndOfNextProductionRound(3 * time.Minute)
			if err != nil {
				cmd.Return(fmt.Errorf("timeout waiting for production round: %w", err))
				return nil
			}
		}

		o.zlogger.Info("issuing 'reload' now")
		emptied := false
		for !emptied {
			select {
			case interimCmd := <-o.commandChan:
				o.zlogger.Info("emptying command queue while safely_reload was running, dropped", zap.Any("interim_cmd", interimCmd))
			default:
				emptied = true
			}
		}

		return o.runSubCommand("reload", cmd)

	case "start", "resume":
		o.zlogger.Info("preparing for start")
		if o.Superviser.IsRunning() {
			o.zlogger.Info("chain is already running")
			return nil
		}

		o.zlogger.Info("preparing to start chain")

		var options []nodeManager.StartOption
		if value := cmd.params["debug-firehose-logs"]; value != "" {
			if value == "true" {
				options = append(options, nodeManager.EnableDebugDeepmindOption)
			} else {
				options = append(options, nodeManager.DisableDebugDeepmindOption)
			}
		}

		if err := o.Superviser.Start(options...); err != nil {
			return fmt.Errorf("error starting chain superviser: %w", err)
		}

		o.zlogger.Info("successfully start service")

	}

	return nil
}

func (c *Command) Return(err error) {
	c.closer.Do(func() {
		if err != nil && err != ErrCleanExit {
			c.logger.Error("command failed", zap.String("cmd", c.cmd), zap.Error(err))
		}

		if c.returnch != nil {
			c.returnch <- err
		}
	})
}

func (o *Operator) LaunchBackupSchedules() {
	for _, sched := range o.backupSchedules {
		if sched.RequiredHostnameMatch != "" {
			hostname, err := os.Hostname()
			if err != nil {
				o.zlogger.Error("Disabling automatic backup schedule because requiredHostname is set and cannot retrieve hostname", zap.Error(err))
				continue
			}
			if sched.RequiredHostnameMatch != hostname {
				o.zlogger.Info("Disabling automatic backup schedule because hostname does not match required value",
					zap.String("hostname", hostname),
					zap.String("required_hostname", sched.RequiredHostnameMatch),
					zap.String("backuper_name", sched.BackuperName))
				continue
			}
		}

		cmdParams := map[string]string{"name": sched.BackuperName}

		if sched.TimeBetweenRuns > time.Second { //loose validation of not-zero (I've seen issues with .IsZero())
			o.zlogger.Info("starting time-based schedule for backup",
				zap.Duration("time_between_runs", sched.TimeBetweenRuns),
				zap.String("backuper_name", sched.BackuperName),
			)
			go o.RunEveryPeriod(sched.TimeBetweenRuns, "backup", cmdParams)
		}
		if sched.BlocksBetweenRuns > 0 {
			o.zlogger.Info("starting block-based schedule for backup",
				zap.Int("blocks_between_runs", sched.BlocksBetweenRuns),
				zap.String("backuper_name", sched.BackuperName),
			)
			go o.RunEveryXBlock(uint32(sched.BlocksBetweenRuns), "backup", cmdParams)
		}
	}
}

func (o *Operator) RunEveryPeriod(period time.Duration, commandName string, params map[string]string) {
	for {
		time.Sleep(100 * time.Microsecond)

		if o.Superviser.IsRunning() {
			break
		}
	}

	ticker := time.NewTicker(period).C

	for range ticker {
		if o.Superviser.IsRunning() {
			o.commandChan <- &Command{cmd: commandName, logger: o.zlogger, params: params}
		}
	}
}

func (o *Operator) RunEveryXBlock(freq uint32, commandName string, params map[string]string) {
	var lastHeadReference uint64
	for {
		time.Sleep(1 * time.Second)
		lastSeenBlockNum := o.Superviser.LastSeenBlockNum()
		if lastSeenBlockNum == 0 {
			continue
		}

		if lastHeadReference == 0 {
			lastHeadReference = lastSeenBlockNum
		}

		if lastSeenBlockNum > lastHeadReference+uint64(freq) {
			o.commandChan <- &Command{cmd: commandName, logger: o.zlogger, params: params}
			lastHeadReference = lastSeenBlockNum
		}
	}
}
