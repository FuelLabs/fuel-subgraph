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

package launcher

import (
	"fmt"
	"runtime/debug"
	"sync"
	"time"

	"github.com/streamingfast/shutter"
	"go.uber.org/atomic"
	"go.uber.org/zap"
)

type Launcher struct {
	shutter *shutter.Shutter

	runtime *Runtime
	apps    map[string]App

	appStatus              map[string]AppStatus
	appStatusSubscriptions []*subscription
	appStatusLock          sync.RWMutex

	shutdownDoOnce     sync.Once
	firstShutdownAppID string
	hasBeenSignaled    *atomic.Bool

	logger *zap.Logger
}

func NewLauncher(logger *zap.Logger, absDataDir string) *Launcher {
	l := &Launcher{
		shutter:         shutter.New(),
		apps:            make(map[string]App),
		appStatus:       make(map[string]AppStatus),
		hasBeenSignaled: new(atomic.Bool),
		logger:          logger,
	}

	l.runtime = &Runtime{
		AbsDataDir: absDataDir,
		IsPendingShutdown: func() bool {
			return l.hasBeenSignaled.Load()
		},
	}

	return l
}

func (l *Launcher) SwitchHasBeenSignaledAtomic(other *atomic.Bool) {
	l.hasBeenSignaled = other
}

func (l *Launcher) Close() {
	l.shutter.Shutdown(nil)
}

func (l *Launcher) Launch(appNames []string) error {
	if len(appNames) == 0 {
		return fmt.Errorf("no apps specified")
	}
	// This is done first as a sanity check so we don't launch anything if something is misconfigured
	for _, appID := range appNames {
		appDef, found := AppRegistry[appID]
		if !found {
			return fmt.Errorf("cannot launch un-registered application %q", appID)
		}

		if appDef.InitFunc != nil {
			l.logger.Debug("initialize application", zap.String("app", appID))
			err := appDef.InitFunc(l.runtime)
			if err != nil {
				return fmt.Errorf("unable to initialize app %q: %w", appID, err)
			}
		}
	}

	for _, appID := range appNames {
		appDef := AppRegistry[appID]

		l.StoreAndStreamAppStatus(appID, AppStatusCreated)
		l.logger.Debug("creating application", zap.String("app", appID))

		// We wrap FactoryFunc inside a func to be able to recover from panic
		app, err := func() (app App, err error) {
			defer func() {
				// If err is not nil, it means FactoryFunc finished, so handle possible recover only if err == nil
				if err == nil {
					err = recoverToError(recover())
				}
			}()

			return appDef.FactoryFunc(l.runtime)
		}()

		if err != nil {
			return fmt.Errorf("unable to create app %q: %w", appID, err)
		}

		l.shutter.OnTerminating(func(err error) {
			go app.Shutdown(err)
		})

		l.apps[appDef.ID] = app
	}

	for appID, app := range l.apps {
		if l.shutter.IsTerminating() {
			break
		}

		// run
		go func(appID string, app App) {
			defer (func() {
				// Don't be fooled, this will work only for this very goroutine and its
				// execution. If the app launches other goroutines and one of those fails, this
				// recovery will not be able to recover them and the whole program will panic
				// without ever reaching this point here.
				l.shutdownIfRecoveringFromPanic(appID, recover())
			})()

			l.logger.Debug("launching app", zap.String("app", appID))
			err := app.Run()
			if err != nil {
				l.shutdownDueToApp(appID, err)
			}
		}(appID, app)
	}

	for appID, app := range l.apps {
		if l.shutter.IsTerminating() {
			break
		}

		// watch for shutdown
		go func(appID string, app App) {
			select {
			case <-app.Terminating():
				l.shutdownDueToApp(appID, app.Err())
			case <-l.shutter.Terminating():
			}
		}(appID, app)
	}

	return nil
}

func (l *Launcher) Terminating() <-chan string {

	ch := make(chan string, 1)

	go func() {
		<-l.shutter.Terminating()
		ch <- l.firstShutdownAppID
	}()

	return ch
}

func (l *Launcher) Err() error {
	return l.shutter.Err()
}

// shutdownDueToApp initiates a launcher shutdown process recording the app that initially triggered
// the shutdown and calling the launcher `Shutdown` method with the error. The `err` can be `nil`
// in which case we assume a clean shutdown. Otherwise, we assume a fatal error shutdown and log
// the fatal error.
func (l *Launcher) shutdownDueToApp(appID string, err error) {
	l.shutdownDoOnce.Do(func() { // pretty printing of error causing dfuse shutdown
		l.firstShutdownAppID = appID

		if err != nil {
			l.FatalAppError(appID, err)
		} else {
			l.logger.Info(fmt.Sprintf("app %s triggered clean shutdown", appID))
		}
	})

	l.StoreAndStreamAppStatus(appID, AppStatusStopped)
	l.shutter.Shutdown(err)
}

func (l *Launcher) FatalAppError(app string, err error) {
	msg := fmt.Sprintf("\n################################################################\n"+
		"Fatal error in app %s:\n\n%s"+
		"\n################################################################\n", app, err)

	l.logger.Error(msg)
}

// shutdownIfRecoveringFromPanic is called with the result of `recover()` call in a `defer`
// to handle any panic and shutdowns down the whole launcher if any recovered error was encountered.
func (l *Launcher) shutdownIfRecoveringFromPanic(appID string, recovered interface{}) (shuttindDown bool) {
	if recovered == nil {
		return false
	}

	err := fmt.Errorf("app %q panicked", appID)
	switch v := recovered.(type) {
	case error:
		err = fmt.Errorf("%s: %w\n%s", err.Error(), v, string(debug.Stack()))
	default:
		err = fmt.Errorf("%s: %s\n%s", err.Error(), v, string(debug.Stack()))
	}

	l.shutdownDueToApp(appID, err)

	return true
}

func recoverToError(recovered interface{}) (err error) {
	if recovered == nil {
		return nil
	}

	if v, ok := recovered.(error); ok {
		return v
	}

	return fmt.Errorf("%s", recovered)
}

func (l *Launcher) StoreAndStreamAppStatus(appID string, status AppStatus) {
	l.appStatusLock.Lock()
	defer l.appStatusLock.Unlock()

	l.appStatus[appID] = status

	appInfo := &AppInfo{
		ID:     appID,
		Status: status,
	}

	for _, sub := range l.appStatusSubscriptions {
		sub.Push(appInfo)
	}
}

func (l *Launcher) GetAppStatus(appID string) AppStatus {
	l.appStatusLock.RLock()
	defer l.appStatusLock.RUnlock()

	if v, found := l.appStatus[appID]; found {
		return v
	}

	return AppStatusNotFound
}

func (l *Launcher) GetAppIDs() (resp []string) {
	for appID := range l.apps {
		resp = append(resp, string(appID))
	}
	return resp
}

func (l *Launcher) WaitForTermination() {
	l.logger.Info("waiting for all apps termination...")
	now := time.Now()
	for appID, app := range l.apps {
	innerFor:
		for {
			select {
			case <-app.Terminated():
				l.logger.Debug("app terminated", zap.String("app_id", appID))
				break innerFor
			case <-time.After(1500 * time.Millisecond):
				l.logger.Info(fmt.Sprintf("still waiting for app %q ... %v", appID, time.Since(now).Round(100*time.Millisecond)))
			}
		}
	}
	l.logger.Info("all apps terminated gracefully")
}

func (l *Launcher) SubscribeAppStatus() *subscription {
	chanSize := 500
	sub := newSubscription(l.logger, chanSize)

	l.appStatusLock.Lock()
	defer l.appStatusLock.Unlock()

	l.appStatusSubscriptions = append(l.appStatusSubscriptions, sub)

	l.logger.Debug("app status subscribed")
	return sub
}

func (l *Launcher) UnsubscribeAppStatus(sub *subscription) {
	if sub == nil {
		return
	}

	l.appStatusLock.Lock()
	defer l.appStatusLock.Unlock()

	var filtered []*subscription
	for _, candidate := range l.appStatusSubscriptions {
		// Pointer address comparison
		if candidate != sub {
			filtered = append(filtered, candidate)
		}
	}

	l.appStatusSubscriptions = filtered
}
