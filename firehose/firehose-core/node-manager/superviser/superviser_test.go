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
	"os"
	"testing"
	"time"

	logplugin "github.com/streamingfast/firehose-core/node-manager/log_plugin"
	"github.com/streamingfast/logging"
	"github.com/stretchr/testify/assert"
	"go.uber.org/zap"
)

var infiniteScript = `
	echo "Starting"
	while true; do
		sleep 0.25
		echo "In loop"
	done
`

var zlog = zap.NewNop()

func init() {
	if os.Getenv("DEBUG") != "" || os.Getenv("TRACE") == "true" {
		zlog, _ := zap.NewDevelopment()
		logging.Override(zlog)
	}
}

var waitDefaultTimeout = 500 * time.Millisecond

func TestSuperviser_NotRunningAfterCreation(t *testing.T) {
	assert.Equal(t, false, testSuperviserInfinite().IsRunning())
}

func TestSuperviser_StartsCorrectly(t *testing.T) {
	superviser := testSuperviserInfinite()
	defer superviser.Stop()

	lineChan := make(chan string)
	superviser.RegisterLogPlugin(logplugin.LogPluginFunc(func(line string) {
		lineChan <- line
	}))

	go superviser.Start()

	waitForSuperviserTaskCompletion(superviser)
	waitForOutput(t, lineChan, waitDefaultTimeout)

	assert.Equal(t, true, superviser.IsRunning())
}

func TestSuperviser_CanBeRestartedCorrectly(t *testing.T) {
	superviser := testSuperviserInfinite()
	defer superviser.Stop()

	lineChan := make(chan string)
	superviser.RegisterLogPlugin(logplugin.LogPluginFunc(func(line string) {
		lineChan <- line
	}))

	go superviser.Start()
	waitForSuperviserTaskCompletion(superviser)
	waitForOutput(t, lineChan, waitDefaultTimeout)

	superviser.Stop()
	assert.Equal(t, false, superviser.IsRunning())

	go superviser.Start()
	waitForSuperviserTaskCompletion(superviser)
	waitForOutput(t, lineChan, waitDefaultTimeout)

	assert.Equal(t, true, superviser.IsRunning())
}

func TestSuperviser_CapturesStdoutCorrectly(t *testing.T) {
	superviser := testSuperviserSh("echo first; sleep 0.1; echo second")
	defer superviser.Stop()

	lineChan := make(chan string)
	superviser.RegisterLogPlugin(logplugin.LogPluginFunc(func(line string) {
		lineChan <- line
	}))

	go superviser.Start()
	waitForSuperviserTaskCompletion(superviser)

	var lines []string
	lines = append(lines, waitForOutput(t, lineChan, waitDefaultTimeout))
	lines = append(lines, waitForOutput(t, lineChan, waitDefaultTimeout))

	assert.Equal(t, []string{"first", "second"}, lines)
}

func testSuperviserBash(script string) *Superviser {
	return New(zlog, "bash", []string{"-c", script})
}

func testSuperviserSh(script string) *Superviser {
	return New(zlog, "sh", []string{"-c", script})
}

func testSuperviserInfinite() *Superviser {
	return testSuperviserSh(infiniteScript)
}

func waitForSuperviserTaskCompletion(superviser *Superviser) {
	superviser.cmdLock.Lock()
	superviser.cmdLock.Unlock()
}

func waitForOutput(t *testing.T, lineChan chan string, timeout time.Duration) (line string) {
	select {
	case line = <-lineChan:
		return
	case <-time.After(timeout):
		t.Error("no line seen before timeout")
	}

	// Will fail before reaching this line
	return ""
}
