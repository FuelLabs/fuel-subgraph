package types

import (
	"fmt"
	"math"
	"strings"

	"github.com/dustin/go-humanize"
)

type BlockNum int64

var HeadBlockNum BlockNum = -1

func (b BlockNum) String() string {
	if b < 0 {
		if b == HeadBlockNum {
			return "HEAD"
		}

		return fmt.Sprintf("HEAD - %d", uint64(math.Abs(float64(b))))
	}

	return "#" + strings.ReplaceAll(humanize.Comma(int64(b)), ",", " ")
}
