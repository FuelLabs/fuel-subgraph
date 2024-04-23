package battlefield

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"

	"github.com/streamingfast/cli"
	"github.com/stretchr/testify/assert"
	"go.uber.org/zap"
)

func CompareBlockFiles(referenceBlockFile, otherBlockFile string, processFileContent func(cntA, cntB []byte) (interface{}, interface{}, error), logger *zap.Logger) (bool, error) {
	logger.Info("comparing block files",
		zap.String("reference_block_file", referenceBlockFile),
		zap.String("other_block_file", otherBlockFile),
	)

	refCnt, err := os.ReadFile(referenceBlockFile)
	if err != nil {
		return false, fmt.Errorf("unable to read block file %q: %w", referenceBlockFile, err)
	}

	otherCnt, err := os.ReadFile(otherBlockFile)
	if err != nil {
		return false, fmt.Errorf("unable to read block file %q: %w", otherBlockFile, err)
	}

	var refBlocksJsonInterface, otherBlocksJsonInterface interface{}
	if processFileContent == nil {
		if err = json.Unmarshal(refCnt, &refBlocksJsonInterface); err != nil {
			return false, fmt.Errorf("unable to unmarshal block %q: %w", referenceBlockFile, err)
		}

		if err = json.Unmarshal(otherCnt, &otherBlocksJsonInterface); err != nil {
			return false, fmt.Errorf("unable to unmarshal block %q: %w", otherBlockFile, err)
		}
	} else {
		refBlocksJsonInterface, otherBlocksJsonInterface, err = processFileContent(refCnt, otherCnt)
		if err != nil {
			return false, fmt.Errorf("failed to process blocks content file: %w", err)
		}
	}

	if assert.ObjectsAreEqualValues(refBlocksJsonInterface, otherBlocksJsonInterface) {
		fmt.Println("Files are equal, all good")
		return true, nil
	}

	useBash := true
	command := fmt.Sprintf("diff -C 5 \"%s\" \"%s\" | less", referenceBlockFile, otherBlockFile)
	if os.Getenv("DIFF_EDITOR") != "" {
		command = fmt.Sprintf("%s \"%s\" \"%s\"", os.Getenv("DIFF_EDITOR"), referenceBlockFile, otherBlockFile)
	}

	showDiff, wasAnswered := cli.AskConfirmation(`File %q and %q differs, do you want to see the difference now`, referenceBlockFile, otherBlockFile)
	if wasAnswered && showDiff {
		diffCmd := exec.Command(command)
		if useBash {
			diffCmd = exec.Command("bash", "-c", command)
		}

		diffCmd.Stdout = os.Stdout
		diffCmd.Stderr = os.Stderr

		if err := diffCmd.Run(); err != nil {
			return false, fmt.Errorf("diff command failed to run properly")
		}

		fmt.Println("You can run the following command to see it manually later:")
	} else {
		fmt.Println("Not showing diff between files, run the following command to see it manually:")
	}

	fmt.Println()
	fmt.Printf("    %s\n", command)
	fmt.Println("")
	return false, nil
}
