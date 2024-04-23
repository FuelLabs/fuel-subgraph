package firecore

import (
	"archive/tar"
	"context"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"time"

	"github.com/streamingfast/dstore"
	"go.uber.org/zap"
)

func NewTarballReaderNodeBootstrapper(
	url string,
	dataDir string,
	logger *zap.Logger,
) *TarballNodeBootstrapper {
	return &TarballNodeBootstrapper{
		url:     url,
		dataDir: dataDir,
		logger:  logger,
	}
}

type TarballNodeBootstrapper struct {
	url     string
	dataDir string
	logger  *zap.Logger
}

func (b *TarballNodeBootstrapper) isBootstrapped() bool {
	return isBootstrapped(b.dataDir, b.logger)
}

func (b *TarballNodeBootstrapper) Bootstrap() error {
	if b.isBootstrapped() {
		return nil
	}

	b.logger.Info("bootstrapping native node chain data from pre-built archive", zap.String("bootstrap_data_url", b.url))

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Minute)
	defer cancel()

	reader, _, _, err := dstore.OpenObject(ctx, b.url, dstore.Compression("zstd"))
	if err != nil {
		return fmt.Errorf("cannot get snapshot from gstore: %w", err)
	}
	defer reader.Close()

	b.createChainData(reader)
	return nil
}

func (b *TarballNodeBootstrapper) createChainData(reader io.Reader) error {
	err := os.MkdirAll(b.dataDir, os.ModePerm)
	if err != nil {
		return fmt.Errorf("unable to create blocks log file: %w", err)
	}

	b.logger.Info("extracting bootstrapping data into node data directory", zap.String("data_dir", b.dataDir))
	tr := tar.NewReader(reader)
	for {
		header, err := tr.Next()
		if err != nil {
			if err == io.EOF {
				return nil
			}

			return err
		}

		path := filepath.Join(b.dataDir, header.Name)
		b.logger.Debug("about to write content of entry", zap.String("name", header.Name), zap.String("path", path), zap.Bool("is_dir", header.FileInfo().IsDir()))
		if header.FileInfo().IsDir() {
			err = os.MkdirAll(path, os.ModePerm)
			if err != nil {
				return fmt.Errorf("unable to create directory: %w", err)
			}

			continue
		}

		file, err := os.Create(path)
		if err != nil {
			return fmt.Errorf("unable to create file: %w", err)
		}

		if _, err := io.Copy(file, tr); err != nil {
			file.Close()
			return err
		}
		file.Close()
	}
}
