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

package merger

import (
	"context"
	"errors"
	"time"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

type Merger struct {
	*shutter.Shutter
	grpcListenAddr string

	io                   IOInterface
	firstStreamableBlock uint64
	logger               *zap.Logger

	timeBetweenPolling time.Duration

	timeBetweenPruning   time.Duration
	pruningDistanceToLIB uint64

	bundler *Bundler
}

func NewMerger(
	logger *zap.Logger,
	grpcListenAddr string,
	io IOInterface,

	firstStreamableBlock uint64,
	bundleSize uint64,
	pruningDistanceToLIB uint64,
	timeBetweenPruning time.Duration,
	timeBetweenPolling time.Duration,
	stopBlock uint64,
) *Merger {
	m := &Merger{
		Shutter:              shutter.New(),
		bundler:              NewBundler(firstStreamableBlock, stopBlock, firstStreamableBlock, bundleSize, io),
		grpcListenAddr:       grpcListenAddr,
		io:                   io,
		firstStreamableBlock: firstStreamableBlock,
		pruningDistanceToLIB: pruningDistanceToLIB,
		timeBetweenPolling:   timeBetweenPolling,
		timeBetweenPruning:   timeBetweenPruning,
		logger:               logger,
	}
	m.OnTerminating(func(_ error) { m.bundler.inProcess.Lock(); m.bundler.inProcess.Unlock() }) // finish bundle that may be merging async

	return m
}

func (m *Merger) Run() {
	m.logger.Info("starting merger")

	m.startGRPCServer()

	m.startOldFilesPruner()
	m.startForkedBlocksPruner()

	err := m.run()
	if err != nil {
		m.logger.Error("merger returned error", zap.Error(err))
	}
	m.Shutdown(err)
}

func (m *Merger) startForkedBlocksPruner() {
	forkableIO, ok := m.io.(ForkAwareIOInterface)
	if !ok {
		return
	}
	m.logger.Info("starting pruning of forked files",
		zap.Uint64("pruning_distance_to_lib", m.pruningDistanceToLIB),
		zap.Duration("time_between_pruning", m.timeBetweenPruning),
	)

	go func() {
		delay := m.timeBetweenPruning // do not start pruning immediately
		for {
			time.Sleep(delay)
			now := time.Now()

			pruningTarget := m.pruningTarget(m.pruningDistanceToLIB)
			forkableIO.DeleteForkedBlocksAsync(bstream.GetProtocolFirstStreamableBlock, pruningTarget)

			if spentTime := time.Since(now); spentTime < m.timeBetweenPruning {
				delay = m.timeBetweenPruning - spentTime
			}
		}
	}()

}

func (m *Merger) startOldFilesPruner() {
	m.logger.Info("starting pruning of unused (old) one-block-files",
		zap.Uint64("pruning_distance_to_lib", m.bundler.bundleSize),
		zap.Duration("time_between_pruning", m.timeBetweenPruning),
	)
	go func() {
		delay := m.timeBetweenPruning // do not start pruning immediately

		unfinishedDelay := time.Second * 5
		if unfinishedDelay > delay {
			unfinishedDelay = delay / 2
		}

		ctx := context.Background()
		for {
			time.Sleep(delay)

			var toDelete []*bstream.OneBlockFile

			pruningTarget := m.pruningTarget(m.bundler.bundleSize)
			if pruningTarget == 0 {
				m.logger.Debug("skipping file deletion until we have a pruning target")
				continue
			}

			delay = m.timeBetweenPruning
			err := m.io.WalkOneBlockFiles(ctx, m.firstStreamableBlock, func(obf *bstream.OneBlockFile) error {
				if obf.Num < pruningTarget {
					toDelete = append(toDelete, obf)
				}
				if len(toDelete) >= DefaultFilesDeleteBatchSize {
					delay = unfinishedDelay
					return ErrStopBlockReached
				}
				return nil
			})
			if err != nil && !errors.Is(err, ErrStopBlockReached) {
				m.logger.Warn("error while walking oneBlockFiles", zap.Error(err))
			}

			m.io.DeleteAsync(toDelete)
		}
	}()
}

func (m *Merger) pruningTarget(distance uint64) uint64 {
	bundlerBase := m.bundler.BaseBlockNum()
	if distance > bundlerBase {
		return 0
	}

	return bundlerBase - distance
}

func (m *Merger) run() error {
	ctx := context.Background()

	var holeFoundLogged bool
	for {
		now := time.Now()
		if m.IsTerminating() {
			return nil
		}

		base, lib, err := m.io.NextBundle(ctx, m.bundler.baseBlockNum)
		if err != nil {
			if errors.Is(err, ErrHoleFound) {
				if holeFoundLogged {
					m.logger.Debug("found hole in merged files. this is not normal behavior unless reprocessing batches", zap.Error(err))
				} else {
					holeFoundLogged = true
					m.logger.Warn("found hole in merged files (next occurrence will show up as Debug)", zap.Error(err))
				}
			} else {
				return err
			}
		}

		if m.bundler.stopBlock != 0 && base > m.bundler.stopBlock {
			if err == ErrStopBlockReached {
				m.logger.Info("stop block reached")
				return nil
			}
		}

		if base > m.bundler.baseBlockNum {
			logFields := []zapcore.Field{
				zap.Uint64("previous_base_block_num", m.bundler.baseBlockNum),
				zap.Uint64("new_base_block_num", base),
			}
			if lib != nil {
				logFields = append(logFields, zap.Stringer("lib", lib))
			}
			m.logger.Info("resetting bundler base block num", logFields...)
			m.bundler.Reset(base, lib)
		}

		var walkErr error
		retryErr := Retry(m.logger, 12, 5*time.Second, func() error {
			err = m.io.WalkOneBlockFiles(ctx, m.bundler.baseBlockNum, func(obf *bstream.OneBlockFile) error {
				return m.bundler.HandleBlockFile(obf)
			})

			if err == ErrFirstBlockAfterInitialStreamableBlock {
				m.bundler.Reset(base, lib)
				return err
			}

			if err != nil {
				walkErr = err
			}
			return nil
		})

		if retryErr != nil {
			return retryErr
		}

		if walkErr != nil {
			if walkErr == ErrStopBlockReached {
				m.logger.Info("stop block reached")
				return nil
			}
			return walkErr
		}

		if spentTime := time.Since(now); spentTime < m.timeBetweenPolling {
			time.Sleep(m.timeBetweenPolling - spentTime)
		}
	}
}
