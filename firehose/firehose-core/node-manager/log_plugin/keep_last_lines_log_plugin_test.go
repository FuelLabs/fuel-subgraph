// Copyright 2019 dfuse Platform Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package logplugin

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestKeepLastLinesLogPlugin(t *testing.T) {
	tests := []struct {
		name                 string
		in                   []string
		maxLine              int
		includeDeepMindLines bool
		out                  []string
	}{
		{"empty", []string{}, 3, false, nil},
		{"single, not reached", []string{"a"}, 3, false, []string{"a"}},
		{"flush, not reached", []string{"a", "b", "c"}, 3, false, []string{"a", "b", "c"}},
		{"over, count", []string{"a", "b", "c", "d"}, 3, false, []string{"b", "c", "d"}},
		{"multiple over count", []string{"a", "b", "c", "d", "e", "f", "g"}, 3, false, []string{"e", "f", "g"}},

		{"max count 0 keeps nothing", []string{"a", "b", "c", "d", "e", "f", "g"}, 0, false, nil},

		{"dm exclude, multiple over count", []string{"a", "b", "DMLOG a", "c", "d", "e", "f", "g", "DMLOG b"}, 3, false, []string{"e", "f", "g"}},
		{"dm include, multiple over count", []string{"a", "b", "DMLOG a", "c", "d", "e", "f", "g", "DMLOG b"}, 3, true, []string{"f", "g", "DMLOG b"}},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			plugin := NewKeepLastLinesLogPlugin(test.maxLine, test.includeDeepMindLines)

			for _, line := range test.in {
				plugin.LogLine(line)
			}

			assert.Equal(t, test.out, plugin.LastLines())
		})
	}
}
