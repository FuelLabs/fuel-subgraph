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

	pbhealth "google.golang.org/grpc/health/grpc_health_v1"
)

// Check is basic GRPC Healthcheck
func (m *Merger) Check(ctx context.Context, in *pbhealth.HealthCheckRequest) (*pbhealth.HealthCheckResponse, error) {
	status := pbhealth.HealthCheckResponse_SERVING
	return &pbhealth.HealthCheckResponse{
		Status: status,
	}, nil
}

// Watch is basic GRPC Healthcheck as a stream
func (m *Merger) Watch(req *pbhealth.HealthCheckRequest, stream pbhealth.Health_WatchServer) error {
	err := stream.Send(&pbhealth.HealthCheckResponse{
		Status: pbhealth.HealthCheckResponse_SERVING,
	})
	if err != nil {
		return err
	}

	// The merger is always serving, so just want until this stream is canceled out
	<-stream.Context().Done()
	return nil
}
