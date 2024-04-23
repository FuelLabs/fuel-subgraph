package firecore

import (
	"context"
	"fmt"

	"github.com/spf13/viper"
	"github.com/streamingfast/dstore"
	"go.uber.org/zap"
)

var commonStoresCreated bool
var indexStoreCreated bool

func GetCommonStoresURLs(dataDir string) (mergedBlocksStoreURL, oneBlocksStoreURL, forkedBlocksStoreURL string, err error) {
	mergedBlocksStoreURL = MustReplaceDataDir(dataDir, viper.GetString("common-merged-blocks-store-url"))
	oneBlocksStoreURL = MustReplaceDataDir(dataDir, viper.GetString("common-one-block-store-url"))
	forkedBlocksStoreURL = MustReplaceDataDir(dataDir, viper.GetString("common-forked-blocks-store-url"))

	if commonStoresCreated {
		return
	}

	if err = mkdirStorePathIfLocal(forkedBlocksStoreURL); err != nil {
		return
	}

	if err = mkdirStorePathIfLocal(oneBlocksStoreURL); err != nil {
		return
	}

	if err = mkdirStorePathIfLocal(mergedBlocksStoreURL); err != nil {
		return
	}

	commonStoresCreated = true
	return
}

func GetIndexStore(dataDir string) (indexStore dstore.Store, possibleIndexSizes []uint64, err error) {
	indexStoreURL := MustReplaceDataDir(dataDir, viper.GetString("common-index-store-url"))

	if indexStoreURL != "" {
		s, err := dstore.NewStore(indexStoreURL, "", "", false)
		if err != nil {
			return nil, nil, fmt.Errorf("couldn't create index store: %w", err)
		}
		if !indexStoreCreated {
			if err = mkdirStorePathIfLocal(indexStoreURL); err != nil {
				return nil, nil, err
			}
		}
		indexStoreCreated = true
		indexStore = s
	}

	for _, size := range viper.GetIntSlice("common-index-block-sizes") {
		if size < 0 {
			return nil, nil, fmt.Errorf("invalid negative size for common-index-block-sizes: %d", size)
		}
		possibleIndexSizes = append(possibleIndexSizes, uint64(size))
	}

	return
}

func LastMergedBlockNum(ctx context.Context, startBlockNum uint64, store dstore.Store, logger *zap.Logger) uint64 {
	value, err := searchBlockNum(startBlockNum, func(u uint64) (bool, error) {
		filepath := fmt.Sprintf("%010d", u)
		found, err := store.FileExists(ctx, filepath)
		if err != nil {
			return false, fmt.Errorf("failed to file exists %s: %w", filepath, err)
		}
		return found, nil
	})
	if err != nil {
		logger.Warn("failed to resolve block", zap.Error(err))
		return startBlockNum
	}
	return value

}

func searchBlockNum(startBlockNum uint64, f func(uint64) (bool, error)) (uint64, error) {
	blockNum, err := blockNumIter(startBlockNum, 10_000_000_000, 1_000_000_000, f)
	if err != nil {
		return 0, err
	}
	if blockNum < startBlockNum {
		return startBlockNum, nil
	}
	return blockNum, nil
}

func blockNumIter(startBlockNum, exclusiveEndBlockNum, interval uint64, f func(uint64) (bool, error)) (uint64, error) {
	i := exclusiveEndBlockNum
	for i >= startBlockNum {
		i -= interval
		match, err := f(i)
		if err != nil {
			return 0, fmt.Errorf("failed to match blcok num %d: %w", i, err)
		}
		if match {
			if interval == 100 {
				return i, nil
			}
			return blockNumIter(i, i+interval, interval/10, f)
		}
	}
	return startBlockNum, nil
}
