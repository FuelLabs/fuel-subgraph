package relayer

import (
	"context"
	"time"

	pbhealth "google.golang.org/grpc/health/grpc_health_v1"
)

func (r *Relayer) Check(ctx context.Context, in *pbhealth.HealthCheckRequest) (*pbhealth.HealthCheckResponse, error) {
	return &pbhealth.HealthCheckResponse{
		Status: r.healthStatus(),
	}, nil
}

func (r *Relayer) Watch(req *pbhealth.HealthCheckRequest, stream pbhealth.Health_WatchServer) error {
	currentStatus := pbhealth.HealthCheckResponse_SERVICE_UNKNOWN
	waitTime := 0 * time.Second

	for {
		select {
		case <-stream.Context().Done():
			return nil
		case <-time.After(waitTime):
			newStatus := r.healthStatus()
			waitTime = 5 * time.Second

			if newStatus != currentStatus {
				currentStatus = newStatus

				if err := stream.Send(&pbhealth.HealthCheckResponse{Status: currentStatus}); err != nil {
					return err
				}
			}
		}
	}
}

func (r *Relayer) healthStatus() pbhealth.HealthCheckResponse_ServingStatus {
	status := pbhealth.HealthCheckResponse_NOT_SERVING
	if r.ready {
		status = pbhealth.HealthCheckResponse_SERVING
	}

	return status
}
