//go:build unix

package shm

import (
	"errors"
	"io"
	"sync/atomic"
	"unsafe"
)

// Block represents a single physical memory block in a Chunk.
type Block struct {
	ChunkID  int16
	BlockIdx int16
	Data     []byte // The mmap'd slice of the chunk for this block
}

// Stripe represents a collection of blocks forming a logical communication unit.
type Stripe struct {
	io.WriterAt
	io.ReaderAt
	Sequence   []int16
	BlockCount int
	Blocks     [][]byte
	DataLen    uint32
}

// GetPointer returns a direct pointer to the memory at a stripe-relative offset.
// It assumes the requested data does not cross block boundaries (BlockSize = 4KB).
func (s *Stripe) GetPointer(offset uintptr) unsafe.Pointer {
	bIdx := offset / BlockSize
	if bIdx >= uintptr(len(s.Blocks)) {
		return nil
	}
	innerOff := offset % BlockSize
	return unsafe.Pointer(&s.Blocks[bIdx][innerOff])
}

func (s *Stripe) updateDataLen(newLen uint32) {
	ptr := unsafe.Pointer(&s.Blocks[0][HEADER_DATA_LEN_OFF])
	atomic.StoreUint32((*uint32)(ptr), newLen)
	s.DataLen = newLen
}

func (s *Stripe) Truncate(newLen uint32) {
	if newLen > s.DataLen {
		return
	}
	s.updateDataLen(newLen)
}

// ReadAt implements io.ReaderAt for a Stripe.
// ... existing ReadAt ...
func (s *Stripe) ReadAt(p []byte, off int64) (n int, err error) {
	if off < 0 {
		return 0, errors.New("negative offset")
	}

	if off >= int64(s.DataLen) {
		return 0, io.EOF
	}

	remaining := int64(len(p))
	if off+remaining > int64(s.DataLen) {
		remaining = int64(s.DataLen) - off
	}
	off += BlockSize // Skip Header Block

	for remaining > 0 {
		bIdx := off / BlockSize
		if bIdx >= int64(len(s.Blocks)) {
			return n, io.EOF
		}
		innerOff := off % BlockSize
		avail := BlockSize - innerOff
		copyLen := avail
		if copyLen > remaining {
			copyLen = remaining
		}

		copy(p[n:n+int(copyLen)], s.Blocks[bIdx][innerOff:innerOff+copyLen])
		n += int(copyLen)
		off += int64(copyLen)
		remaining -= copyLen
	}
	return n, nil
}

// WriteAt implements io.WriterAt for a Stripe.
func (s *Stripe) WriteAt(p []byte, off int64) (n int, err error) {
	if off < 0 {
		return 0, errors.New("negative offset")
	}

	off += BlockSize // Skip Header Block

	remaining := len(p)
	for remaining > 0 {
		bIdx := off / int64(BlockSize)
		if bIdx >= int64(len(s.Blocks)) {
			return n, errors.New("stripe capacity exceeded")
		}
		innerOff := off % int64(BlockSize)
		avail := int64(BlockSize) - innerOff
		copyLen := int(avail)
		if copyLen > remaining {
			copyLen = remaining
		}

		copy(s.Blocks[bIdx][innerOff:innerOff+int64(copyLen)], p[n:n+copyLen])
		n += copyLen
		off += int64(copyLen)
		remaining -= copyLen
	}

	newEnd := uint32(off - BlockSize) // Actual data end offset
	if newEnd > s.DataLen {
		s.updateDataLen(newEnd)
	}

	return n, nil
}
