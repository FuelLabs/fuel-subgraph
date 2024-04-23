package store

import (
	"fmt"
	"math/big"
	"strconv"

	"github.com/shopspring/decimal"
)

func (b *baseStore) SetMaxBigInt(ord uint64, key string, value *big.Int) {
	max := new(big.Int)
	val, found := b.GetAt(ord, key)
	if !found {
		max = value
	} else {
		prev, _ := new(big.Int).SetString(string(val), 10)
		if prev != nil && value.Cmp(prev) > 0 {
			max = value
		} else {
			max = prev
		}
	}
	b.set(ord, key, []byte(max.String()))
}

func (b *baseStore) SetMaxInt64(ord uint64, key string, value int64) {
	var max int64
	val, found := b.GetAt(ord, key)
	if !found {
		max = value
	} else {
		prev, err := strconv.ParseInt(string(val), 10, 64)
		if err != nil || value > prev {
			max = value
		} else {
			max = prev
		}
	}
	b.set(ord, key, []byte(fmt.Sprintf("%d", max)))
}

func (b *baseStore) SetMaxFloat64(ord uint64, key string, value float64) {
	var max float64
	val, found := b.GetAt(ord, key)
	if !found {
		max = value
	} else {
		prev, err := strconv.ParseFloat(string(val), 64)

		if err != nil || value > prev {
			max = value
		} else {
			max = prev
		}
	}
	b.set(ord, key, []byte(strconv.FormatFloat(max, 'g', 100, 64)))
}

func (b *baseStore) SetMaxBigDecimal(ord uint64, key string, value decimal.Decimal) {
	val, found := b.GetAt(ord, key)
	if !found {
		b.set(ord, key, []byte(value.String()))
		return
	}
	prev, err := decimal.NewFromString(string(val))
	prev.Truncate(34)
	if err != nil || value.Cmp(prev) == 1 {
		b.set(ord, key, []byte(value.String()))
		return
	}
	b.set(ord, key, []byte(prev.String()))
}
