package firecore

import "golang.org/x/exp/maps"

var ReaderNodeVariablesDocumentation = map[string]string{
	"{data-dir}":        "The current data-dir path defined by the flag 'data-dir'",
	"{node-data-dir}":   "The node data dir path defined by the flag 'reader-node-data-dir'",
	"{hostname}":        "The machine's hostname",
	"{start-block-num}": "The resolved start block number defined by the flag 'reader-node-start-block-num' (can be overwritten)",
	"{stop-block-num}":  "The stop block number defined by the flag 'reader-node-stop-block-num'",
}

var ReaderNodeVariables = maps.Keys(ReaderNodeVariablesDocumentation)

func ReaderNodeVariablesValues(resolver ReaderNodeArgumentResolver) map[string]string {
	values := make(map[string]string, len(ReaderNodeVariables))
	for _, variable := range ReaderNodeVariables {
		values[variable] = resolver(variable)
	}

	return values
}
