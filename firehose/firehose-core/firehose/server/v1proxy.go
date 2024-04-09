package server

import (
	"fmt"

	pbfirehoseV1 "github.com/streamingfast/pbgo/sf/firehose/v1"
	pbfirehoseV2 "github.com/streamingfast/pbgo/sf/firehose/v2"
	"google.golang.org/grpc"
)

type FirehoseProxyV1ToV2 struct {
	server *Server
}

func NewFirehoseProxyV1ToV2(server *Server) *FirehoseProxyV1ToV2 {
	return &FirehoseProxyV1ToV2{
		server: server,
	}
}

func (s *FirehoseProxyV1ToV2) Blocks(req *pbfirehoseV1.Request, streamSrv pbfirehoseV1.Stream_BlocksServer) error {

	var finalBlocksOnly bool
	var validSteps bool
	var withUndo bool
	switch len(req.ForkSteps) {
	case 1:
		if req.ForkSteps[0] == pbfirehoseV1.ForkStep_STEP_IRREVERSIBLE {
			finalBlocksOnly = true
			validSteps = true
		}
		if req.ForkSteps[0] == pbfirehoseV1.ForkStep_STEP_NEW {
			validSteps = true
		}
	case 2:
		if (req.ForkSteps[0] == pbfirehoseV1.ForkStep_STEP_NEW && req.ForkSteps[1] == pbfirehoseV1.ForkStep_STEP_UNDO) ||
			(req.ForkSteps[1] == pbfirehoseV1.ForkStep_STEP_NEW && req.ForkSteps[0] == pbfirehoseV1.ForkStep_STEP_UNDO) {
			validSteps = true
			withUndo = true
		} else if req.ForkSteps[0] == pbfirehoseV1.ForkStep_STEP_NEW && req.ForkSteps[1] == pbfirehoseV1.ForkStep_STEP_IRREVERSIBLE {
			validSteps = true
			// compatibility hack. you won't receive IRREVERSIBLE here
		}
	}
	if !validSteps {
		return fmt.Errorf("invalid parameter for ForkSteps: this server implements firehose v2 operation and only supports [NEW,UNDO] or [IRREVERSIBLE]")
	}

	reqV2 := &pbfirehoseV2.Request{
		StartBlockNum:   req.StartBlockNum,
		Cursor:          req.StartCursor,
		StopBlockNum:    req.StopBlockNum,
		FinalBlocksOnly: finalBlocksOnly,
		Transforms:      req.Transforms,
	}

	wrapper := streamWrapper{ServerStream: streamSrv, next: streamSrv, withUndo: withUndo}

	return s.server.Blocks(reqV2, wrapper)
}

type streamWrapper struct {
	grpc.ServerStream
	next     pbfirehoseV1.Stream_BlocksServer
	withUndo bool
}

func (w streamWrapper) Send(response *pbfirehoseV2.Response) error {
	return w.next.Send(&pbfirehoseV1.Response{
		Block:  response.Block,
		Step:   convertForkStep(response.Step),
		Cursor: response.Cursor,
	})
}

func convertForkStep(in pbfirehoseV2.ForkStep) pbfirehoseV1.ForkStep {
	switch in {
	case pbfirehoseV2.ForkStep_STEP_FINAL:
		return pbfirehoseV1.ForkStep_STEP_IRREVERSIBLE
	case pbfirehoseV2.ForkStep_STEP_NEW:
		return pbfirehoseV1.ForkStep_STEP_NEW
	case pbfirehoseV2.ForkStep_STEP_UNDO:
		return pbfirehoseV1.ForkStep_STEP_UNDO
	}
	return pbfirehoseV1.ForkStep_STEP_UNKNOWN
}
