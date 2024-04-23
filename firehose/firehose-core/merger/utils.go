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
	"fmt"
	"time"

	"github.com/streamingfast/bstream"
	"go.uber.org/zap"
	"gopkg.in/olivere/elastic.v3/backoff"
)

func fileNameForBlocksBundle(blockNum uint64) string {
	return fmt.Sprintf("%010d", blockNum)
}

func toBaseNum(in uint64, bundleSize uint64) uint64 {
	return in / bundleSize * bundleSize
}

func Retry(logger *zap.Logger, attempts int, sleep time.Duration, function func() error) (err error) {
	b := backoff.NewExponentialBackoff(sleep, 5*time.Second)
	for i := 0; ; i++ {
		err = function()
		if err == nil {
			return
		}

		if i >= (attempts - 1) {
			break
		}

		time.Sleep(b.Next())

		logger.Warn("retrying after error", zap.Error(err))
	}
	return fmt.Errorf("after %d attempts, last error: %s", attempts, err)
}

type TestMergerIO struct {
	NextBundleFunc           func(ctx context.Context, lowestBaseBlock uint64) (baseBlock uint64, lastIrreversibleBlock bstream.BlockRef, err error)
	WalkOneBlockFilesFunc    func(ctx context.Context, inclusiveLowerBlock uint64, callback func(*bstream.OneBlockFile) error) error
	MergeAndStoreFunc        func(ctx context.Context, inclusiveLowerBlock uint64, oneBlockFiles []*bstream.OneBlockFile) (err error)
	DownloadOneBlockFileFunc func(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error)
	DeleteAsyncFunc          func(oneBlockFiles []*bstream.OneBlockFile) error
}

func (io *TestMergerIO) NextBundle(ctx context.Context, lowestBaseBlock uint64) (baseBlock uint64, lastIrreversibleBlock bstream.BlockRef, err error) {
	if io.NextBundleFunc != nil {
		return io.NextBundleFunc(ctx, lowestBaseBlock)
	}
	return lowestBaseBlock, nil, nil
}

func (io *TestMergerIO) MergeAndStore(ctx context.Context, inclusiveLowerBlock uint64, oneBlockFiles []*bstream.OneBlockFile) (err error) {
	if io.MergeAndStoreFunc != nil {
		return io.MergeAndStoreFunc(ctx, inclusiveLowerBlock, oneBlockFiles)
	}
	return nil
}

func (io *TestMergerIO) DownloadOneBlockFile(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error) {
	if io.DownloadOneBlockFileFunc != nil {
		return io.DownloadOneBlockFileFunc(ctx, oneBlockFile)
	}

	return nil, nil
}

func (io *TestMergerIO) WalkOneBlockFiles(ctx context.Context, inclusiveLowerBlock uint64, callback func(*bstream.OneBlockFile) error) error {
	if io.WalkOneBlockFilesFunc != nil {
		return io.WalkOneBlockFilesFunc(ctx, inclusiveLowerBlock, callback)
	}
	return nil
}
func (io *TestMergerIO) DeleteAsync(oneBlockFiles []*bstream.OneBlockFile) error {
	if io.DeleteAsyncFunc != nil {
		return io.DeleteAsyncFunc(oneBlockFiles)
	}
	return nil
}
