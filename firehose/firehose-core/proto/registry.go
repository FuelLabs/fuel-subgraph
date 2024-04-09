package proto

import (
	"errors"
	"fmt"
	"strings"

	"google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/reflect/protoregistry"
	"google.golang.org/protobuf/types/dynamicpb"
	"google.golang.org/protobuf/types/known/anypb"
)

// Generate the flags based on Go code in this project directly, this however
// creates a chicken & egg problem if there is compilation error within the project
// but to fix them we must re-generate it.
//go:generate go run ./generator well_known_types.go proto

type Registry struct {
	Types *protoregistry.Types
	Files *protoregistry.Files
}

func NewRegistry(chainFileDescriptor protoreflect.FileDescriptor, protoPaths ...string) (*Registry, error) {
	r := &Registry{
		Types: new(protoregistry.Types),
		Files: new(protoregistry.Files),
	}
	// Proto paths have the highest precedence, so we register them first
	if len(protoPaths) > 0 {
		if err := r.RegisterFiles(protoPaths); err != nil {
			return nil, fmt.Errorf("register proto files: %w", err)
		}
	}

	// Chain file descriptor has the second highest precedence, it always
	// override built-in types if defined.
	if chainFileDescriptor != nil {
		err := r.RegisterFileDescriptor(chainFileDescriptor)
		if err != nil {
			return nil, fmt.Errorf("register chain file descriptor: %w", err)
		}
	}

	//Last are well known types, they have the lowest precedence
	err := RegisterWellKnownFileDescriptors(r)
	if err != nil {
		return nil, fmt.Errorf("registering well known file descriptors: %w", err)
	}

	return r, nil
}

func (r *Registry) RegisterFiles(files []string) error {
	if len(files) == 0 {
		return nil
	}

	fileDescriptors, err := parseProtoFiles(files)
	if err != nil {
		return fmt.Errorf("parsing proto files: %w", err)
	}

	return r.RegisterFileDescriptors(fileDescriptors)
}

func (r *Registry) RegisterFileDescriptors(fds []protoreflect.FileDescriptor) error {
	for _, fd := range fds {
		err := r.RegisterFileDescriptor(fd)
		if err != nil {
			return fmt.Errorf("registering proto file: %w", err)
		}
	}
	return nil
}
func (r *Registry) RegisterFileDescriptor(fd protoreflect.FileDescriptor) error {
	path := fd.Path()
	_, err := r.Files.FindFileByPath(path)

	if err != nil {
		if errors.Is(err, protoregistry.NotFound) {
			// NewRegistry the new file descriptor.
			if err := r.Files.RegisterFile(fd); err != nil {
				return fmt.Errorf("registering proto file: %w", err)
			}

			// Create a new MessageType using the registered FileDescriptor
			msgCount := fd.Messages().Len()
			for i := 0; i < msgCount; i++ {
				messageType := fd.Messages().Get(i)
				if messageType == nil {
					return fmt.Errorf("message type not found in the registered file")
				}

				dmt := dynamicpb.NewMessageType(messageType) // NewRegistry the MessageType
				err := r.Types.RegisterMessage(dmt)
				if err != nil {
					return fmt.Errorf("registering message type: %w", err)
				}
			}
			return nil
		}
		return fmt.Errorf("finding file by path: %w", err)
	}

	//that mean we already have this file registered, we need to check if we have the message type registered
	return nil
}

func (r *Registry) Unmarshal(a *anypb.Any) (*dynamicpb.Message, error) {
	messageType, err := r.Types.FindMessageByURL(a.TypeUrl)
	if err != nil {
		return nil, fmt.Errorf("failed to find message '%s': %v", urlToMessageFullName(a.TypeUrl), err)
	}

	message := dynamicpb.NewMessage(messageType.Descriptor())
	err = a.UnmarshalTo(message)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal message: %v", err)
	}

	return message, nil
}

func urlToMessageFullName(url string) protoreflect.FullName {
	message := protoreflect.FullName(url)
	if i := strings.LastIndexByte(url, '/'); i >= 0 {
		message = message[i+len("/"):]
	}

	return message
}
