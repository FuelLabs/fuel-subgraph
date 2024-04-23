package merger

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/dstore"
	"github.com/streamingfast/firehose-core/merger/metrics"
	"github.com/streamingfast/logging"
	"go.uber.org/zap"
)

var ErrHoleFound = errors.New("hole found in merged files")
var DefaultFilesDeleteBatchSize = 10000

type IOInterface interface {

	// NextBundle will read through consecutive merged blocks, starting at `lowestBaseBlock`, and return the next bundle that needs to be created
	// If it finds an existing merged file at `lowestBaseBlock`, it will read the last one and include the lastIrreversibleBlock so you can bootstrap your forkdb from there
	NextBundle(ctx context.Context, lowestBaseBlock uint64) (baseBlock uint64, lastIrreversibleBlock bstream.BlockRef, err error)

	// WalkOneBlockFiles calls your function for each oneBlockFile it reads, starting at the inclusiveLowerBlock. Useful to feed a block source
	WalkOneBlockFiles(ctx context.Context, inclusiveLowerBlock uint64, callback func(*bstream.OneBlockFile) error) error

	// MergeAndStore writes a merged file from a list of oneBlockFiles
	MergeAndStore(ctx context.Context, inclusiveLowerBlock uint64, oneBlockFiles []*bstream.OneBlockFile) (err error)

	// DownloadOneBlockFile will get you the data from the file
	DownloadOneBlockFile(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error)

	// DeleteAsync should be able to delete large quantities of oneBlockFiles from storage without ever blocking
	DeleteAsync(oneBlockFiles []*bstream.OneBlockFile) error
}

type ForkAwareIOInterface interface {
	// DeleteForkedBlocksAsync will delete forked blocks between lowBoundary and highBoundary (both inclusive)
	DeleteForkedBlocksAsync(inclusiveLowBoundary, inclusiveHighBoundary uint64)

	// MoveForkedBlocks will copy an array of oneBlockFiles to the forkedBlocksStore, then delete them (dstore does not have MOVE primitive)
	MoveForkedBlocks(ctx context.Context, oneBlockFiles []*bstream.OneBlockFile)
}

type ForkAwareDStoreIO struct {
	*DStoreIO
	forkedBlocksStore dstore.Store
	forkOd            *oneBlockFilesDeleter
}

type DStoreIO struct {
	oneBlocksStore    dstore.Store
	mergedBlocksStore dstore.Store

	retryAttempts int
	retryCooldown time.Duration

	bundleSize uint64

	logger *zap.Logger
	tracer logging.Tracer
	od     *oneBlockFilesDeleter
	forkOd *oneBlockFilesDeleter
}

func NewDStoreIO(
	logger *zap.Logger,
	tracer logging.Tracer,
	oneBlocksStore dstore.Store,
	mergedBlocksStore dstore.Store,
	forkedBlocksStore dstore.Store,
	retryAttempts int,
	retryCooldown time.Duration,
	bundleSize uint64,
	numDeleteThreads int,
) IOInterface {

	od := &oneBlockFilesDeleter{store: oneBlocksStore, logger: logger}
	od.Start(numDeleteThreads, DefaultFilesDeleteBatchSize*2)
	dstoreIO := &DStoreIO{
		oneBlocksStore:    oneBlocksStore,
		mergedBlocksStore: mergedBlocksStore,
		retryAttempts:     retryAttempts,
		retryCooldown:     retryCooldown,
		bundleSize:        bundleSize,
		logger:            logger,
		tracer:            tracer,
		od:                od,
	}

	forkAware := forkedBlocksStore != nil
	if !forkAware {
		return dstoreIO
	}

	forkOd := &oneBlockFilesDeleter{store: forkedBlocksStore, logger: logger}
	forkOd.Start(numDeleteThreads, DefaultFilesDeleteBatchSize*2)

	return &ForkAwareDStoreIO{
		DStoreIO:          dstoreIO,
		forkedBlocksStore: forkedBlocksStore,
		forkOd:            forkOd,
	}
}

func (s *DStoreIO) MergeAndStore(ctx context.Context, inclusiveLowerBlock uint64, oneBlockFiles []*bstream.OneBlockFile) (err error) {
	// since we keep the last block from previous merged bundle for future deleting,
	// we want to make sure that it does not end up in this merged bundle too
	var filteredOBF []*bstream.OneBlockFile

	if len(oneBlockFiles) == 0 {
		return fmt.Errorf("cannot merge and store without a single oneBlockFile")
	}
	anyOneBlockFile := oneBlockFiles[0]
	for _, obf := range oneBlockFiles {
		if obf.Num >= inclusiveLowerBlock {
			filteredOBF = append(filteredOBF, obf)
		}
	}
	t0 := time.Now()

	bundleFilename := fileNameForBlocksBundle(inclusiveLowerBlock)

	zapFields := []zap.Field{
		zap.String("filename", bundleFilename),
		zap.Duration("write_timeout", WriteObjectTimeout),
		zap.Int("number_of_blocks", len(filteredOBF)),
	}
	if len(filteredOBF) != 0 {
		zapFields = append(zapFields, zap.Uint64("lower_block_num", filteredOBF[0].Num), zap.Uint64("highest_block_num", filteredOBF[len(filteredOBF)-1].Num))
	}

	s.logger.Info("about to write merged blocks to storage location", zapFields...)

	err = Retry(s.logger, s.retryAttempts, s.retryCooldown, func() error {
		inCtx, cancel := context.WithTimeout(ctx, WriteObjectTimeout)
		defer cancel()
		bundleReader, err := NewBundleReader(ctx, s.logger, s.tracer, filteredOBF, anyOneBlockFile, s.DownloadOneBlockFile)
		if err != nil {
			return err
		}
		return s.mergedBlocksStore.WriteObject(inCtx, bundleFilename, bundleReader)
	})
	if err != nil {
		return fmt.Errorf("write object error: %s", err)
	}

	s.logger.Info("merged and uploaded", zap.String("filename", fileNameForBlocksBundle(inclusiveLowerBlock)), zap.Duration("merge_time", time.Since(t0)))

	return
}

func (s *DStoreIO) WalkOneBlockFiles(ctx context.Context, lowestBlock uint64, callback func(*bstream.OneBlockFile) error) error {
	return s.oneBlocksStore.WalkFrom(ctx, "", fileNameForBlocksBundle(lowestBlock), func(filename string) error {
		if strings.HasSuffix(filename, ".tmp") {
			return nil
		}
		oneBlockFile := bstream.MustNewOneBlockFile(filename)

		if err := callback(oneBlockFile); err != nil {
			return err
		}
		return nil
	})

}

// fixLegacyBlock reads the header and looks for "Version 0", rewriting to Version 1 on the fly if needed
func fixLegacyBlock(in []byte) ([]byte, error) {
	dbinReader, err := bstream.NewDBinBlockReader(bytes.NewReader(in))
	if err != nil {
		return nil, fmt.Errorf("creating block reader in fixLegacyBlock: %w", err)
	}

	if dbinReader.Header.Version != 0 {
		return in, nil
	}

	reader, err := bstream.NewDBinBlockReader(bytes.NewReader(in))
	if err != nil {
		return nil, fmt.Errorf("creating block reader in fixLegacyBlock: %w", err)
	}

	blk, err := reader.Read()
	if err != nil {
		return nil, fmt.Errorf("reading block in fixLegacyBlock: %w", err)
	}

	out := new(bytes.Buffer)
	writer, err := bstream.NewDBinBlockWriter(out)
	if err != nil {
		return nil, err
	}

	if err := writer.Write(blk); err != nil {
		return nil, fmt.Errorf("writing block in fixLegacyBlock: %w", err)
	}
	return out.Bytes(), nil

}

func (s *DStoreIO) DownloadOneBlockFile(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error) {
	for filename := range oneBlockFile.Filenames { // will try to get MemoizeData from any of those files
		var out io.ReadCloser
		out, err = s.oneBlocksStore.OpenObject(ctx, filename)
		s.logger.Debug("downloading one block", zap.String("file_name", filename))
		if err != nil {
			continue
		}
		defer out.Close()

		select {
		case <-ctx.Done():
			return nil, ctx.Err()
		default:
		}

		data, err = io.ReadAll(out)
		if err != nil {
			continue
		}

		data, err = fixLegacyBlock(data)
		if err == nil {
			break
		}
	}

	return
}

func (s *DStoreIO) NextBundle(ctx context.Context, lowestBaseBlock uint64) (outBaseBlock uint64, lib bstream.BlockRef, err error) {
	var lastFound *uint64
	outBaseBlock = lowestBaseBlock
	err = s.mergedBlocksStore.WalkFrom(ctx, "", fileNameForBlocksBundle(lowestBaseBlock), func(filename string) error {
		num, err := strconv.ParseUint(filename, 10, 64)
		if err != nil {
			return err
		}

		if num != outBaseBlock {
			return fmt.Errorf("%w: merged blocks skip from %d to %d, you need to fill this hole, set firstStreamableBlock above this hole or set merger option to ignore holes", ErrHoleFound, outBaseBlock, num)
		}
		outBaseBlock += s.bundleSize
		lastFound = &num
		return nil
	})

	if lastFound != nil {
		last, lastTime, err := s.readLastBlockFromMerged(ctx, *lastFound)
		if err != nil {
			return 0, nil, err
		}
		metrics.HeadBlockTimeDrift.SetBlockTime(*lastTime)
		metrics.HeadBlockNumber.SetUint64(last.Num())
		lib = last
	}

	return
}

func (s *DStoreIO) readLastBlockFromMerged(ctx context.Context, baseBlock uint64) (bstream.BlockRef, *time.Time, error) {
	subCtx, cancel := context.WithTimeout(ctx, GetObjectTimeout)
	defer cancel()
	reader, err := s.mergedBlocksStore.OpenObject(subCtx, fileNameForBlocksBundle(baseBlock))
	if err != nil {
		return nil, nil, err
	}
	last, err := lastBlock(reader)
	if err != nil {
		return nil, nil, err
	}
	// we truncate the block ID to have the short version that we get on oneBlockFiles
	t := last.Timestamp.AsTime()
	return bstream.NewBlockRef(bstream.TruncateBlockID(last.Id), last.Number), &t, nil
}

func (s *DStoreIO) DeleteAsync(oneBlockFiles []*bstream.OneBlockFile) error {
	return s.od.Delete(oneBlockFiles)
}

func (s *ForkAwareDStoreIO) MoveForkedBlocks(ctx context.Context, oneBlockFiles []*bstream.OneBlockFile) {
	for _, f := range oneBlockFiles {
		for name := range f.Filenames {
			reader, err := s.oneBlocksStore.OpenObject(ctx, name)
			if err != nil {
				s.logger.Warn("could not copy forked block", zap.Error(err))
				continue
			}
			err = s.forkedBlocksStore.WriteObject(ctx, name, reader)
			if err != nil {
				s.logger.Warn("could not copy forked block", zap.Error(err))
				continue
			}
			reader.Close()
			break
		}
	}
	_ = s.od.Delete(oneBlockFiles)
}

func (s *ForkAwareDStoreIO) DeleteForkedBlocksAsync(inclusiveLowBoundary, inclusiveHighBoundary uint64) {
	var forkedBlockFiles []*bstream.OneBlockFile
	err := s.forkedBlocksStore.WalkFrom(context.Background(), "", "", func(filename string) error {
		if strings.HasSuffix(filename, ".tmp") {
			return nil
		}
		obf := bstream.MustNewOneBlockFile(filename)
		if obf.Num > inclusiveHighBoundary {
			return io.EOF
		}
		forkedBlockFiles = append(forkedBlockFiles, obf)
		return nil
	})

	if err != nil && err != io.EOF {
		s.logger.Warn("cannot walk forked block files to delete old ones",
			zap.Uint64("inclusive_low_boundary", inclusiveLowBoundary),
			zap.Uint64("inclusive_high_boundary", inclusiveHighBoundary),
			zap.Error(err),
		)
	}

	s.forkOd.Delete(forkedBlockFiles)
}

type oneBlockFilesDeleter struct {
	sync.Mutex
	toProcess     chan string
	retryAttempts int
	retryCooldown time.Duration
	store         dstore.Store
	logger        *zap.Logger
}

func (od *oneBlockFilesDeleter) Start(threads int, maxDeletions int) {
	od.toProcess = make(chan string, maxDeletions)
	for i := 0; i < threads; i++ {
		go od.processDeletions()
	}
}

func (od *oneBlockFilesDeleter) Delete(oneBlockFiles []*bstream.OneBlockFile) error {
	od.Lock()
	defer od.Unlock()

	if len(oneBlockFiles) == 0 {
		return nil
	}

	var fileNames []string
	for _, oneBlockFile := range oneBlockFiles {
		for filename := range oneBlockFile.Filenames {
			fileNames = append(fileNames, filename)
		}
	}
	od.logger.Info("deleting a bunch of one_block_files", zap.Int("number_of_files", len(fileNames)), zap.String("first_file", fileNames[0]), zap.String("last_file", fileNames[len(fileNames)-1]), zap.Stringer("store", od.store.BaseURL()))

	deletable := make(map[string]bool)

	for _, f := range fileNames {
		deletable[f] = true
	}

	// dedupe processing queue
	for empty := false; !empty; {
		select {
		case f := <-od.toProcess:
			deletable[f] = true
		default:
			empty = true
		}
	}

	var deletableArr []string
	for file := range deletable {
		deletableArr = append(deletableArr, file)
	}
	sort.Strings(deletableArr)

	var err error
	for _, file := range deletableArr {
		if len(od.toProcess) == cap(od.toProcess) {
			od.logger.Warn("skipping file deletions: the channel is full", zap.Int("capacity", cap(od.toProcess)))
			err = fmt.Errorf("skipped some files")
			break
		}
		od.toProcess <- file
	}
	return err
}

func (od *oneBlockFilesDeleter) processDeletions() {
	for {
		file := <-od.toProcess
		err := Retry(od.logger, od.retryAttempts, od.retryCooldown, func() error {
			ctx, cancel := context.WithTimeout(context.Background(), DeleteObjectTimeout)
			defer cancel()
			err := od.store.DeleteObject(ctx, file)
			if errors.Is(err, dstore.ErrNotFound) {
				return nil
			}
			return err
		})
		if err != nil {
			od.logger.Warn("cannot delete oneblock file after a few retries", zap.String("file", file), zap.Error(err))
		}
	}
}

func lastBlock(mergeFileReader io.ReadCloser) (out *pbbstream.Block, err error) {
	defer mergeFileReader.Close()

	blkReader, err := bstream.NewDBinBlockReader(mergeFileReader)
	if err != nil {
		return nil, err
	}

	for {
		block, err := blkReader.Read()
		if block != nil {
			out = block
		}
		if err != nil {
			if err == io.EOF {
				break
			}
			return nil, err
		}
	}

	return out, nil
}
