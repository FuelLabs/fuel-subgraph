package launcher

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseFromArgs(t *testing.T) {
	tests := []struct {
		name   string
		input  []string
		expect []string
	}{
		{
			input:  []string{"all,-app2"},
			expect: []string{"app1", "app3"},
		},
		{
			input:  []string{"all", " -app2 "},
			expect: []string{"app1", "app3"},
		},
		{
			input:  []string{"all"},
			expect: []string{"app1", "app2", "app3"},
		},
		{
			input:  []string{" app1", " app2"},
			expect: []string{"app1", "app2"},
		},
		{
			input:  []string{" app1, app2"},
			expect: []string{"app1", "app2"},
		},
		{
			input:  []string{"app2", "appnodefault"},
			expect: []string{"app2", "appnodefault"},
		},
	}

	RegisterApp(zlog, &AppDef{ID: "app1"})
	RegisterApp(zlog, &AppDef{ID: "app2"})
	RegisterApp(zlog, &AppDef{ID: "app3"})
	RegisterApp(zlog, &AppDef{ID: "appnodefault"})

	for idx, test := range tests {
		t.Run(fmt.Sprintf("%d", idx), func(t *testing.T) {
			res := ParseAppsFromArgs(test.input, func(app string) bool {
				if app == "appnodefault" {
					return false
				}
				return true
			})
			assert.Equal(t, test.expect, res)
		})
	}
}
