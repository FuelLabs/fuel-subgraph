package merger

import (
	dgrpcfactory "github.com/streamingfast/dgrpc/server/factory"
	pbhealth "google.golang.org/grpc/health/grpc_health_v1"
)

func (m *Merger) startGRPCServer() {
	gs := dgrpcfactory.ServerFromOptions()
	gs.OnTerminated(m.Shutdown)
	m.logger.Info("grpc server created")

	m.OnTerminated(func(_ error) {
		gs.Shutdown(0)
	})
	pbhealth.RegisterHealthServer(gs.ServiceRegistrar(), m)
	m.logger.Info("server registered")

	go gs.Launch(m.grpcListenAddr)

}
