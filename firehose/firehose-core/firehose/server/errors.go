package server

import (
	"fmt"
)

type ErrSendBlock struct {
	inner error
}

func NewErrSendBlock(inner error) ErrSendBlock {
	return ErrSendBlock{
		inner: inner,
	}
}

func (e ErrSendBlock) Error() string {
	return fmt.Sprintf("send error: %s", e.inner)
}
