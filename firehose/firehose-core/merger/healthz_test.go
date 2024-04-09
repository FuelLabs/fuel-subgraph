package merger

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/require"
	pbhealth "google.golang.org/grpc/health/grpc_health_v1"
)

func TestHealthz_Check(t *testing.T) {
	ctx := context.Background()
	m := NewMerger(
		testLogger,
		"6969",
		nil,
		1,
		100,
		100,
		time.Second,
		time.Second,
		0,
	)
	request := &pbhealth.HealthCheckRequest{}
	resp, err := m.Check(ctx, request)
	if err != nil {
		panic(err)
	}

	require.Equal(t, resp.Status, pbhealth.HealthCheckResponse_SERVING)
}
