package firecore

import (
	"context"
	"fmt"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/hub"
	"github.com/streamingfast/bstream/stream"
	"github.com/streamingfast/bstream/transform"
	"github.com/streamingfast/dauth"
	"github.com/streamingfast/dmetering"
	"github.com/streamingfast/dstore"
	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v2"
	"go.uber.org/zap"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

var StreamMergedBlocksPreprocThreads = 25

type StreamFactory struct {
	mergedBlocksStore dstore.Store
	forkedBlocksStore dstore.Store
	hub               *hub.ForkableHub
	transformRegistry *transform.Registry
}

func NewStreamFactory(
	mergedBlocksStore dstore.Store,
	forkedBlocksStore dstore.Store,
	hub *hub.ForkableHub,
	transformRegistry *transform.Registry,
) *StreamFactory {
	return &StreamFactory{
		mergedBlocksStore: mergedBlocksStore,
		forkedBlocksStore: forkedBlocksStore,
		hub:               hub,
		transformRegistry: transformRegistry,
	}
}

func (sf *StreamFactory) New(
	ctx context.Context,
	handler bstream.Handler,
	request *pbfirehose.Request,
	logger *zap.Logger) (*stream.Stream, error) {

	reqLogger := logger.With(
		zap.Int64("req_start_block", request.StartBlockNum),
		zap.String("req_cursor", request.Cursor),
		zap.Uint64("req_stop_block", request.StopBlockNum),
		zap.Bool("final_blocks_only", request.FinalBlocksOnly),
	)

	options := []stream.Option{
		stream.WithStopBlock(request.StopBlockNum),
	}

	preprocFunc, blockIndexProvider, desc, err := sf.transformRegistry.BuildFromTransforms(request.Transforms)
	if err != nil {
		reqLogger.Error("cannot process incoming blocks request transforms", zap.Error(err))
		return nil, fmt.Errorf("building from transforms: %w", err)
	}
	if preprocFunc != nil {
		options = append(options, stream.WithPreprocessFunc(preprocFunc, StreamMergedBlocksPreprocThreads))
	}
	if blockIndexProvider != nil {
		reqLogger = reqLogger.With(zap.Bool("with_index_provider", true))
	}
	if desc != "" {
		reqLogger = reqLogger.With(zap.String("transform_desc", desc))
	}
	options = append(options, stream.WithLogger(logger)) // stream won't have the full reqLogger, use the traceID to connect them together

	if blockIndexProvider != nil {
		options = append(options, stream.WithBlockIndexProvider(blockIndexProvider))
	}

	if request.FinalBlocksOnly {
		options = append(options, stream.WithFinalBlocksOnly())
	}

	var fields []zap.Field
	auth := dauth.FromContext(ctx)
	if auth != nil {
		fields = append(fields,
			zap.String("api_key_id", auth.APIKeyID()),
			zap.String("user_id", auth.UserID()),
			zap.String("real_ip", auth.RealIP()),
		)
		if auth["x-deployment-id"] != "" {
			fields = append(fields, zap.String("deployment_id", auth["x-deployment-id"]))
		}
	}

	reqLogger.Info("processing incoming blocks request", fields...)

	if request.Cursor != "" {
		cur, err := bstream.CursorFromOpaque(request.Cursor)
		if err != nil {
			return nil, status.Errorf(codes.InvalidArgument, "invalid start cursor %q: %s", request.Cursor, err)
		}

		options = append(options, stream.WithCursor(cur))
	}

	forkedBlocksStore := sf.forkedBlocksStore
	if clonable, ok := forkedBlocksStore.(dstore.Clonable); ok {
		var err error
		forkedBlocksStore, err = clonable.Clone(ctx)
		if err != nil {
			return nil, err
		}
		forkedBlocksStore.SetMeter(dmetering.GetBytesMeter(ctx))
	}

	mergedBlocksStore := sf.mergedBlocksStore
	if clonable, ok := mergedBlocksStore.(dstore.Clonable); ok {
		var err error
		mergedBlocksStore, err = clonable.Clone(ctx)
		if err != nil {
			return nil, err
		}
		mergedBlocksStore.SetMeter(dmetering.GetBytesMeter(ctx))
	}

	str := stream.New(
		forkedBlocksStore,
		mergedBlocksStore,
		sf.hub,
		request.StartBlockNum,
		handler,
		options...)

	return str, nil
}
