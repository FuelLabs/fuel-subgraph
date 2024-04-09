package proto

import (
	"encoding/hex"
	"fmt"

	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/reflect/protodesc"
	"google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/types/descriptorpb"
)

type WellKnownType struct {
	proto         string
	BytesEncoding string
}

func RegisterWellKnownFileDescriptors(registry *Registry) error {

	for _, wt := range wellKnownTypes {
		fd, err := protoToFileDescriptor(registry, wt.proto)
		if err != nil {
			return fmt.Errorf("generating proto file: %w", err)
		}
		err = registry.RegisterFileDescriptor(fd)
		if err != nil {
			return fmt.Errorf("registering file descriptor: %w", err)
		}

	}
	return nil
}

func protoToFileDescriptor(registry *Registry, in string) (protoreflect.FileDescriptor, error) {
	protoBytes, err := hex.DecodeString(in)
	if err != nil {
		panic(fmt.Errorf("failed to hex decode payload: %w", err))
	}

	fileDescriptorProto := &descriptorpb.FileDescriptorProto{}
	if err := proto.Unmarshal(protoBytes, fileDescriptorProto); err != nil {
		return nil, fmt.Errorf("failed to unmarshal file descriptor: %w", err)
	}

	fd, err := protodesc.NewFile(fileDescriptorProto, registry.Files)
	if err != nil {
		return nil, fmt.Errorf("creating new file descriptor: %w", err)

	}
	return fd, nil
}
