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
	"sync"

	"go.uber.org/zap"
)

type subscription struct {
	IncomingAppInfo chan *AppInfo
	Closed          bool
	QuitOnce        sync.Once

	logger *zap.Logger
}

func newSubscription(logger *zap.Logger, chanSize int) (out *subscription) {
	return &subscription{
		IncomingAppInfo: make(chan *AppInfo, chanSize),
	}
}

func (s *subscription) Push(app *AppInfo) {
	if s.Closed {
		return
	}

	s.logger.Debug("pushing app readiness state to subscriber",
		zap.Reflect("response", app),
	)
	if len(s.IncomingAppInfo) == cap(s.IncomingAppInfo) {
		s.QuitOnce.Do(func() {
			s.logger.Debug("reach max buffer size for readiness stream, closing channel")
			close(s.IncomingAppInfo)
			s.Closed = true
		})
		return
	}

	// Clean up
	s.IncomingAppInfo <- app
}
