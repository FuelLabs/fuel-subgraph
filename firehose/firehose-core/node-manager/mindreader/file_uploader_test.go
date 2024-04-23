package mindreader

import (
	"context"
	"testing"
	"time"

	"github.com/streamingfast/dstore"
	"github.com/stretchr/testify/require"
)

func TestFileUploader(t *testing.T) {
	localStore := dstore.NewMockStore(nil)
	localStore.SetFile("test1", nil)
	localStore.SetFile("test2", nil)
	localStore.SetFile("test3", nil)

	destinationStore := dstore.NewMockStore(nil)

	done := make(chan interface{})
	out := make(chan bool, 3)

	destinationStore.PushLocalFileFunc = func(_ context.Context, _, _ string) (err error) {
		out <- true
		return nil
	}
	go func() {
		for i := 0; i < 3; i++ {
			<-out
		}
		close(done)
	}()

	uploader := NewFileUploader(localStore, destinationStore, testLogger)
	err := uploader.uploadFiles(context.Background())
	require.NoError(t, err)

	select {
	case <-done:
	case <-time.After(5 * time.Second):
		t.Error("took took long")
	}
}
