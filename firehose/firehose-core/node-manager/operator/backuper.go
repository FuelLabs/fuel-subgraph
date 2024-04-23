package operator

import (
	"fmt"
	"reflect"
	"strconv"
	"strings"
	"time"

	"go.uber.org/zap"
)

type BackupModuleConfig map[string]string

type BackupModuleFactory func(conf BackupModuleConfig) (BackupModule, error)

type BackupModule interface {
	Backup(lastSeenBlockNum uint32) (string, error)
	RequiresStop() bool
}

type RestorableBackupModule interface {
	BackupModule
	Restore(name string) error
}

type BackupSchedule struct {
	BlocksBetweenRuns     int
	TimeBetweenRuns       time.Duration
	RequiredHostnameMatch string // will not run backup if !empty env.Hostname != HostnameMatch
	BackuperName          string // must match id of backupModule
}

func (o *Operator) RegisterBackupModule(name string, mod BackupModule) error {
	if o.backupModules == nil {
		o.backupModules = make(map[string]BackupModule)
	}

	if existing, found := o.backupModules[name]; found {
		return fmt.Errorf("backup module %q is already registered, previous module type %s", name, reflect.ValueOf(existing))
	}

	o.backupModules[name] = mod
	return nil
}

func (o *Operator) RegisterBackupSchedule(sched *BackupSchedule) {
	o.backupSchedules = append(o.backupSchedules, sched)
}

func selectBackupModule(mods map[string]BackupModule, optionalName string) (BackupModule, error) {
	if len(mods) == 0 {
		return nil, fmt.Errorf("no registered backup modules")
	}

	if optionalName != "" {
		chosen, ok := mods[optionalName]
		if !ok {
			return nil, fmt.Errorf("invalid backup module: %s", optionalName)
		}
		return chosen, nil
	}

	if len(mods) > 1 {
		var modNames []string
		for k := range mods {
			modNames = append(modNames, k)
		}
		return nil, fmt.Errorf("more than one module registered, and none specified (%s)", strings.Join(modNames, ","))
	}

	for _, mod := range mods { // single element in map
		return mod, nil
	}
	return nil, fmt.Errorf("impossible path")

}

func selectRestoreModule(choices map[string]BackupModule, optionalName string) (RestorableBackupModule, error) {
	mods := restorable(choices)
	if len(mods) == 0 {
		return nil, fmt.Errorf("none of the registered backup modules support 'restore'")
	}

	if optionalName != "" {
		chosen, ok := mods[optionalName]
		if !ok {
			return nil, fmt.Errorf("invalid restorable backup module: %s", optionalName)
		}
		return chosen, nil
	}

	if len(mods) > 1 {
		var modNames []string
		for k := range mods {
			modNames = append(modNames, k)
		}
		return nil, fmt.Errorf("more than one restorable module registered, and none specified (%s)", strings.Join(modNames, ","))
	}

	for _, mod := range mods { // single element in map
		return mod, nil
	}
	return nil, fmt.Errorf("impossible path")

}

func restorable(in map[string]BackupModule) map[string]RestorableBackupModule {
	out := make(map[string]RestorableBackupModule)
	for k, v := range in {
		if rest, ok := v.(RestorableBackupModule); ok {
			out[k] = rest
		}
	}
	return out
}

func NewBackupSchedule(freqBlocks, freqTime, requiredHostname, backuperName string) (*BackupSchedule, error) {
	switch {
	case freqBlocks != "":
		freqUint, err := strconv.ParseUint(freqBlocks, 10, 64)
		if err != nil || freqUint == 0 {
			return nil, fmt.Errorf("invalid value for freq_block in backup schedule (err: %w)", err)
		}

		return &BackupSchedule{
			BlocksBetweenRuns:     int(freqUint),
			RequiredHostnameMatch: requiredHostname,
			BackuperName:          backuperName,
		}, nil

	case freqTime != "":
		freqTime, err := time.ParseDuration(freqTime)
		if err != nil || freqTime < time.Minute {
			return nil, fmt.Errorf("invalid value for freq_time in backup schedule(duration: %s, err: %w)", freqTime, err)
		}

		return &BackupSchedule{
			TimeBetweenRuns:       freqTime,
			RequiredHostnameMatch: requiredHostname,
			BackuperName:          backuperName,
		}, nil

	default:
		return nil, fmt.Errorf("schedule created without any frequency value")
	}
}

func ParseBackupConfigs(
	logger *zap.Logger,
	backupConfigs []string,
	backupModuleFactories map[string]BackupModuleFactory,
) (
	mods map[string]BackupModule,
	scheds []*BackupSchedule,
	err error,
) {
	logger.Info("parsing backup configs", zap.Strings("configs", backupConfigs), zap.Int("factory_count", len(backupModuleFactories)))
	for key := range backupModuleFactories {
		logger.Info("parsing backup known factory", zap.String("name", key))
	}

	mods = make(map[string]BackupModule)
	for _, confStr := range backupConfigs {
		conf, err := parseKVConfigString(confStr)
		if err != nil {
			return nil, nil, err
		}

		t := conf["type"]
		factory, found := backupModuleFactories[t]
		if !found {
			return nil, nil, fmt.Errorf("unknown backup module type %q", t)
		}

		mods[t], err = factory(conf)
		if err != nil {
			return nil, nil, fmt.Errorf("backup module %q factory: %w", t, err)
		}

		if conf["freq-blocks"] != "" || conf["freq-time"] != "" {
			newSched, err := NewBackupSchedule(conf["freq-blocks"], conf["freq-time"], conf["required-hostname"], t)
			if err != nil {
				return nil, nil, fmt.Errorf("error setting up backup schedule for %q: %w", t, err)
			}

			scheds = append(scheds, newSched)
		}
	}

	return
}

// parseKVConfigString is used for flags that generate key/value data, like
// `--backup="type=something freq_blocks=1000 prefix=v1"`.
func parseKVConfigString(in string) (map[string]string, error) {
	fields := strings.Fields(in)
	kvs := map[string]string{}
	for _, field := range fields {
		kv := strings.Split(field, "=")
		if len(kv) != 2 {
			return nil, fmt.Errorf("invalid key=value in kv config string: %s", field)
		}
		kvs[kv[0]] = kv[1]
	}
	typ, ok := kvs["type"]
	if !ok || typ == "" {
		return nil, fmt.Errorf("no type defined in kv config string (type field mandatory)")
	}

	return kvs, nil
}
