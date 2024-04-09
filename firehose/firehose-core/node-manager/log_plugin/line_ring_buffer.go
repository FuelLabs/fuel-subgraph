package logplugin

type bufferElement struct {
	previous *bufferElement
	next     *bufferElement
	line     string
}

type lineRingBuffer struct {
	maxCount int

	count int
	tail  *bufferElement
	head  *bufferElement
}

func (b *lineRingBuffer) lines() (out []string) {
	if b.count == 0 {
		return nil
	}

	if b.count == 1 {
		return []string{b.head.line}
	}

	i := 0
	out = make([]string, b.count)
	for current := b.tail; current != nil; current = current.next {
		out[i] = current.line
		i++
	}

	return
}

func (b *lineRingBuffer) append(line string) {
	// If we keep nothing, there is nothing to do here
	if b.maxCount == 0 {
		return
	}

	oldHead := b.head
	b.head = &bufferElement{line: line, previous: oldHead}

	if oldHead != nil {
		oldHead.next = b.head
	}

	if b.tail == nil {
		b.tail = b.head
	}

	if b.count == b.maxCount {
		// We are full, we need to rotate stuff a bit
		b.tail = b.tail.next
	} else {
		// We are not full, let's just append a new line (so only update count)
		b.count++
	}
}
