package merger

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"io/ioutil"
	"path"
	"testing"
	"time"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/dbin"
	"github.com/stretchr/testify/require"
)

func TestBundleReader_ReadSimpleFiles(t *testing.T) {
	bundle := NewTestBundle()

	r, err := NewBundleReader(context.Background(), testLogger, testTracer, bundle, bundle[0], nil)
	require.NoError(t, err)

	r1 := make([]byte, len(testOneBlockHeader))
	read, err := r.Read(r1)
	require.NoError(t, err, "reading header")
	require.Equal(t, string(testOneBlockHeader), string(r1))

	r1 = make([]byte, 2)
	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 2, read)
	require.Equal(t, []byte{0x1, 0x2}, r1)

	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 2, read)
	require.Equal(t, []byte{0x3, 0x4}, r1)

	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 2, read)
	require.Equal(t, []byte{0x5, 0x6}, r1)

	read, err = r.Read(r1)
	require.Equal(t, 0, read)
	require.Equal(t, io.EOF, err)
}

func TestBundleReader_ReadByChunk(t *testing.T) {
	bundle := NewTestBundle()

	r, err := NewBundleReader(context.Background(), testLogger, testTracer, bundle, bundle[0], nil)
	require.NoError(t, err)

	r1 := make([]byte, len(testOneBlockHeader))
	read, err := r.Read(r1)
	require.NoError(t, err, "reading header")
	require.Equal(t, string(testOneBlockHeader), string(r1))

	r1 = make([]byte, 1)
	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 1, read)
	require.Equal(t, []byte{0x1}, r1)

	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 1, read)
	require.Equal(t, []byte{0x2}, r1)

	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 1, read)
	require.Equal(t, []byte{0x3}, r1)

	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 1, read)
	require.Equal(t, []byte{0x4}, r1)

	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 1, read)
	require.Equal(t, []byte{0x5}, r1)

	read, err = r.Read(r1)
	require.NoError(t, err)
	require.Equal(t, 1, read)
	require.Equal(t, []byte{0x6}, r1)

	_, err = r.Read(r1)
	require.Equal(t, err, io.EOF)
}

func TestBundleReader_Read_Then_Read_Block(t *testing.T) {
	bundle := []*bstream.OneBlockFile{
		NewTestOneBlockFileFromFile(t, "0000000001-20150730T152628.0-13406cb6-b1cb8fa3.dbin"),
		NewTestOneBlockFileFromFile(t, "0000000002-20150730T152657.0-044698c9-13406cb6.dbin"),
		NewTestOneBlockFileFromFile(t, "0000000003-20150730T152728.0-a88cf741-044698c9.dbin"),
	}

	r, err := NewBundleReader(context.Background(), testLogger, testTracer, bundle, bundle[0], nil)
	require.NoError(t, err)
	allBlockData, err := ioutil.ReadAll(r)
	require.NoError(t, err)
	dbinReader := dbin.NewReader(bytes.NewReader(allBlockData))

	//Reader header once
	_, err = dbinReader.ReadHeader()

	//Block 1
	require.NoError(t, err)
	b1, err := dbinReader.ReadMessage()
	require.NoError(t, err)
	require.Equal(t, b1, bundle[0].MemoizeData[14:])

	//Block 2
	require.NoError(t, err)
	b2, err := dbinReader.ReadMessage()
	require.NoError(t, err)
	require.Equal(t, b2, bundle[1].MemoizeData[14:])

	//Block 3
	require.NoError(t, err)
	b3, err := dbinReader.ReadMessage()
	require.NoError(t, err)
	require.Equal(t, b3, bundle[2].MemoizeData[14:])
}

func TestBundleReader_Read_DownloadOneBlockFileError(t *testing.T) {
	bundle := NewBundleNoMemoize()
	anyOB := &bstream.OneBlockFile{
		CanonicalName: "header",
		MemoizeData:   testOneBlockHeader,
	}

	downloadOneBlockFile := func(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error) {
		return nil, fmt.Errorf("some error")
	}
	r, err := NewBundleReader(context.Background(), testLogger, testTracer, bundle, anyOB, downloadOneBlockFile)
	require.NoError(t, err)

	r1 := make([]byte, len(testOneBlockHeader))
	read, err := r.Read(r1)
	require.NoError(t, err, "reading header")
	require.Equal(t, string(testOneBlockHeader), string(r1))

	read, err = r.Read(r1)
	require.Equal(t, 0, read)
	require.Errorf(t, err, "some error")
}

func TestBundleReader_Read_DownloadOneBlockFileCorrupt(t *testing.T) {

	bundle := NewBundleNoMemoize()

	downloadOneBlockFile := func(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error) {
		return []byte{0xAB, 0xCD, 0xEF}, nil // shorter than header length
	}

	_, err := NewBundleReader(context.Background(), testLogger, testTracer, bundle, bundle[0], downloadOneBlockFile)
	require.Error(t, err)
}

func TestBundleReader_Read_DownloadOneBlockFileZeroLength(t *testing.T) {
	bundle := NewBundleNoMemoize()

	anyBlockFile := &bstream.OneBlockFile{
		CanonicalName: "header",
		MemoizeData:   testOneBlockHeader,
	}

	downloadOneBlockFile := func(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error) {
		return []byte{}, nil
	}

	r, err := NewBundleReader(context.Background(), testLogger, testTracer, bundle, anyBlockFile, downloadOneBlockFile)
	require.NoError(t, err)

	r1 := make([]byte, len(testOneBlockHeader))
	read, err := r.Read(r1)
	require.NoError(t, err, "reading header")
	require.Equal(t, string(testOneBlockHeader), string(r1))

	r1 = make([]byte, 4)
	read, err = r.Read(r1)
	require.Equal(t, read, 0)
	require.Error(t, err, "EOF expected")
}

func TestBundleReader_Read_ReadBufferNotNil(t *testing.T) {
	bundle := NewBundleNoMemoize()

	anyBlockFile := &bstream.OneBlockFile{
		CanonicalName: "header",
		MemoizeData:   testOneBlockHeader,
	}

	downloadOneBlockFile := func(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error) {
		return nil, fmt.Errorf("some error")
	}

	r, err := NewBundleReader(context.Background(), testLogger, testTracer, bundle, anyBlockFile, downloadOneBlockFile)
	require.NoError(t, err)
	r.readBuffer = []byte{0xAB, 0xCD}
	r1 := make([]byte, 4)

	read, err := r.Read(r1)
	require.Equal(t, read, 2)
	require.Nil(t, err)
}

func TestBundleReader_Read_EmptyListOfOneBlockFiles(t *testing.T) {
	bundle := NewBundleNoMemoize()

	anyBlockFile := &bstream.OneBlockFile{
		CanonicalName: "header",
		MemoizeData:   testOneBlockHeader,
	}

	downloadOneBlockFile := func(ctx context.Context, oneBlockFile *bstream.OneBlockFile) (data []byte, err error) {
		return nil, fmt.Errorf("some error")
	}

	r, err := NewBundleReader(context.Background(), testLogger, testTracer, bundle, anyBlockFile, downloadOneBlockFile)
	require.NoError(t, err)

	r1 := make([]byte, len(testOneBlockHeader))
	read, err := r.Read(r1)
	require.NoError(t, err, "reading header")
	require.Equal(t, string(testOneBlockHeader), string(r1))

	r1 = make([]byte, 4)
	read, err = r.Read(r1)
	require.Equal(t, 0, read)
	require.Errorf(t, err, "EOF")
}

func NewTestOneBlockFileFromFile(t *testing.T, fileName string) *bstream.OneBlockFile {
	t.Helper()
	data, err := ioutil.ReadFile(path.Join("test_data", fileName))
	require.NoError(t, err)
	time.Sleep(1 * time.Millisecond)
	return &bstream.OneBlockFile{
		CanonicalName: fileName,
		Filenames:     map[string]bool{fileName: true},
		ID:            "",
		Num:           0,
		PreviousID:    "",
		MemoizeData:   data,
	}
}

var testOneBlockHeader = []byte("dbin\x00tes\x00\x00")

func NewTestBundle() []*bstream.OneBlockFile {

	o1 := &bstream.OneBlockFile{
		CanonicalName: "o1",
		MemoizeData:   append(testOneBlockHeader, []byte{0x1, 0x2}...),
	}
	o2 := &bstream.OneBlockFile{
		CanonicalName: "o2",
		MemoizeData:   append(testOneBlockHeader, []byte{0x3, 0x4}...),
	}
	o3 := &bstream.OneBlockFile{
		CanonicalName: "o3",
		MemoizeData:   append(testOneBlockHeader, []byte{0x5, 0x6}...),
	}
	return []*bstream.OneBlockFile{o1, o2, o3}
}

func NewBundleNoMemoize() []*bstream.OneBlockFile {
	o1 := &bstream.OneBlockFile{
		CanonicalName: "o1",
		MemoizeData:   []byte{},
	}
	o2 := &bstream.OneBlockFile{
		CanonicalName: "o2",
		MemoizeData:   []byte{},
	}
	o3 := &bstream.OneBlockFile{
		CanonicalName: "o3",
		MemoizeData:   []byte{},
	}
	return []*bstream.OneBlockFile{o1, o2, o3}
}
