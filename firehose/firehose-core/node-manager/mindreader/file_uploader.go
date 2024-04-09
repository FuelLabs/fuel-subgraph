package mindreader

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/abourget/llerrgroup"
	"github.com/streamingfast/dstore"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
)

type FileUploader struct {
	*shutter.Shutter
	mutex            sync.Mutex
	localStore       dstore.Store
	destinationStore dstore.Store
	logger           *zap.Logger
	complete         chan struct{}
}

func NewFileUploader(localStore dstore.Store, destinationStore dstore.Store, logger *zap.Logger) *FileUploader {
	return &FileUploader{
		Shutter:          shutter.New(),
		complete:         make(chan struct{}),
		localStore:       localStore,
		destinationStore: destinationStore,
		logger:           logger,
	}
}

func (fu *FileUploader) Start(ctx context.Context) {
	defer close(fu.complete)

	fu.OnTerminating(func(_ error) {
		<-fu.complete
	})

	if fu.IsTerminating() {
		return
	}

	var terminating bool
	for {
		err := fu.uploadFiles(ctx)
		if err != nil {
			fu.logger.Warn("failed to upload file", zap.Error(err))
		}

		if terminating {
			return
		}

		select {
		case <-fu.Terminating():
			fu.logger.Info("terminating upload loop on next pass")
			terminating = true
		case <-time.After(500 * time.Millisecond):
		}
	}
}

func (fu *FileUploader) uploadFiles(ctx context.Context) error {
	fu.mutex.Lock()
	defer fu.mutex.Unlock()

	eg := llerrgroup.New(200)
	_ = fu.localStore.Walk(ctx, "", func(filename string) (err error) {
		if eg.Stop() {
			return nil
		}
		eg.Go(func() error {
			ctx, cancel := context.WithTimeout(context.Background(), 3*time.Minute)
			defer cancel()

			if traceEnabled {
				fu.logger.Debug("uploading file to storage", zap.String("local_file", filename))
			}

			if err = fu.destinationStore.PushLocalFile(ctx, fu.localStore.ObjectPath(filename), filename); err != nil {
				return fmt.Errorf("moving file %q to storage: %w", filename, err)
			}
			return nil
		})

		return nil
	})

	return eg.Wait()
}
