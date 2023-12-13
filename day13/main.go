package day13

import (
	"strings"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	patternBlocks := strings.Split(strings.Join(lines, "\n"), "\n\n")
	patterns := make([][][]rune, len(patternBlocks))

	for i, pattern := range patternBlocks {
		patternLines := strings.Split(pattern, "\n")
		newPattern := make([][]rune, len(patternLines))

		for j, line := range patternLines {
			newLine := make([]rune, len(line))
			for k, c := range line {
				newLine[k] = c
			}
			newPattern[j] = newLine
		}

		patterns[i] = newPattern
	}

	return patterns
}

func (d Day) SolvePart1(patternsI interface{}) (amount int) {
	patterns := patternsI.([][][]rune)

	cols := 0
	rows := 0
	for _, pattern := range patterns {
		rows += findMirror(pattern, false)
		cols += findMirror(convertIntoCols(pattern), false)
	}

	return cols + 100*rows
}

func (d Day) SolvePart2(patternsI interface{}) int {
	patterns := patternsI.([][][]rune)

	cols := 0
	rows := 0
	for _, pattern := range patterns {
		rows += findMirror(pattern, true)
		cols += findMirror(convertIntoCols(pattern), true)
	}

	return cols + 100*rows
}
