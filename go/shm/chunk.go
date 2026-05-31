//go:build unix

package shm

import (
	"syscall"

	"golang.org/x/sys/unix"
)

// --- Chunk Support ---

type Chunk struct {
	Data []byte
	Fd   int
}

func NewChunk(name string) (*Chunk, error) {
	fd, err := unix.MemfdCreate(name, unix.MFD_CLOEXEC|unix.MFD_ALLOW_SEALING)
	if err != nil {
		return nil, err
	}
	fdNum := int(fd)
	if err := syscall.Ftruncate(fdNum, int64(ChunkSize)); err != nil {
		syscall.Close(fdNum)
		return nil, err
	}
	chunk, err := AttachChunk(fdNum)
	if err != nil {
		syscall.Close(fdNum)
		return nil, err
	}
	return chunk, nil
}

func AttachChunk(fd int) (*Chunk, error) {
	data, err := syscall.Mmap(fd, 0, ChunkSize, syscall.PROT_READ|syscall.PROT_WRITE, syscall.MAP_SHARED)
	if err != nil {
		return nil, err
	}
	return &Chunk{Data: data, Fd: fd}, nil
}

func (c *Chunk) Close() error {
	if c == nil {
		return nil
	}
	if c.Data != nil {
		syscall.Munmap(c.Data)
		c.Data = nil
	}
	if c.Fd >= 0 {
		err := syscall.Close(c.Fd)
		c.Fd = -1
		return err
	}
	return nil
}
