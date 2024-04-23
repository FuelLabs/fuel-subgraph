package launcher

import (
	"fmt"
	"os"

	"gopkg.in/yaml.v2"
)

var Config map[string]*CommandConfig

type CommandConfig struct {
	Args  []string          `json:"args"`
	Flags map[string]string `json:"flags"`
}

// Load reads a YAML config, and sets the global DfuseConfig variable
// Use the raw JSON form to provide to the
// different plugins and apps for them to load their config.
func LoadConfigFile(filename string) (err error) {
	yamlBytes, err := os.ReadFile(filename)
	if err != nil {
		return err
	}

	err = yaml.Unmarshal(yamlBytes, &Config)
	if err != nil {
		return fmt.Errorf("reading json: %s", err)
	}

	return nil
}
