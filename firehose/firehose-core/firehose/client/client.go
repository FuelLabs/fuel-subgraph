package client

import (
	"context"
	"crypto/tls"
	"fmt"

	"github.com/streamingfast/dgrpc"
	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v2"
	"golang.org/x/oauth2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/credentials/oauth"
	"google.golang.org/grpc/metadata"
)

// firehoseClient, closeFunc, grpcCallOpts, err := NewFirehoseClient(endpoint, jwt, insecure, plaintext)
// defer closeFunc()
// stream, err := firehoseClient.Blocks(context.Background(), request, grpcCallOpts...)
func NewFirehoseClient(endpoint, jwt, apiKey string, useInsecureTSLConnection, usePlainTextConnection bool) (cli pbfirehose.StreamClient, closeFunc func() error, callOpts []grpc.CallOption, err error) {

	if useInsecureTSLConnection && usePlainTextConnection {
		return nil, nil, nil, fmt.Errorf("option --insecure and --plaintext are mutually exclusive, they cannot be both specified at the same time")
	}

	var dialOptions []grpc.DialOption
	switch {
	case usePlainTextConnection:
		dialOptions = []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}

	case useInsecureTSLConnection:
		dialOptions = []grpc.DialOption{grpc.WithTransportCredentials(credentials.NewTLS(&tls.Config{InsecureSkipVerify: true}))}
	}

	conn, err := dgrpc.NewExternalClient(endpoint, dialOptions...)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("unable to create external gRPC client: %w", err)
	}
	closeFunc = conn.Close
	cli = pbfirehose.NewStreamClient(conn)

	if !usePlainTextConnection {
		if jwt != "" {
			credentials := oauth.NewOauthAccess(&oauth2.Token{AccessToken: jwt, TokenType: "Bearer"})
			callOpts = append(callOpts, grpc.PerRPCCredentials(credentials))
		} else if apiKey != "" {
			callOpts = append(callOpts, grpc.PerRPCCredentials(&ApiKeyAuth{ApiKey: apiKey}))
		}
	}

	return
}

type ApiKeyAuth struct {
	ApiKey string
}

func (a *ApiKeyAuth) GetRequestMetadata(ctx context.Context, uri ...string) (out map[string]string, err error) {
	md, ok := metadata.FromOutgoingContext(ctx)
	if !ok {
		md = metadata.New(nil)
	}
	out = make(map[string]string)
	for k, v := range md {
		if len(v) != 0 {
			out[k] = v[0]
		}
	}
	if a.ApiKey != "" {
		out["x-api-key"] = a.ApiKey
	}
	return
}

func (a *ApiKeyAuth) RequireTransportSecurity() bool {
	return true
}

func NewFirehoseFetchClient(endpoint, jwt, apiKey string, useInsecureTSLConnection, usePlainTextConnection bool) (cli pbfirehose.FetchClient, closeFunc func() error, callOpts []grpc.CallOption, err error) {

	if useInsecureTSLConnection && usePlainTextConnection {
		return nil, nil, nil, fmt.Errorf("option --insecure and --plaintext are mutually exclusive, they cannot be both specified at the same time")
	}

	var dialOptions []grpc.DialOption
	switch {
	case usePlainTextConnection:
		dialOptions = []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}

	case useInsecureTSLConnection:
		dialOptions = []grpc.DialOption{grpc.WithTransportCredentials(credentials.NewTLS(&tls.Config{InsecureSkipVerify: true}))}
	}

	if !usePlainTextConnection {
		if jwt != "" {
			credentials := oauth.NewOauthAccess(&oauth2.Token{AccessToken: jwt, TokenType: "Bearer"})
			callOpts = append(callOpts, grpc.PerRPCCredentials(credentials))
		} else if apiKey != "" {
			callOpts = append(callOpts, grpc.PerRPCCredentials(&ApiKeyAuth{ApiKey: apiKey}))
		}
	}

	conn, err := dgrpc.NewExternalClient(endpoint, dialOptions...)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("unable to create external gRPC client: %w", err)
	}
	closeFunc = conn.Close
	cli = pbfirehose.NewFetchClient(conn)

	return
}
