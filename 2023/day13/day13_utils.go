package day13

import (
	"golang.org/x/exp/slices"
)

func lineDiffs(s1, s2 []rune) (diffs int) {
	if slices.Equal(s1, s2) {
		return 0
	}

	for i := 0; i < len(s1); i++ {
		if s1[i] != s2[i] {
			diffs++
		}
	}

	return
}

func compareLines(s1 []rune, s2 []rune) (int, bool) {
	if !slices.Equal(s1, s2) {
		return lineDiffs(s1, s2), false
	}

	return 0, true
}

func areEqualSlices(ss1 [][]rune, ss2 [][]rune) (int, bool) {
	differences := 0
	for i := 0; i < len(ss1); i++ {
		curDiffs, same := compareLines(ss1[i], ss2[i])
		if !same {
			differences += curDiffs
		}
	}

	return differences, differences == 0
}

func findMirror(pattern [][]rune, part2 bool) int {
	for i := 1; i < len(pattern); i++ {
		unseen := pattern[i:]
		// var seen [][]rune
		seen := slices.Clone(pattern[:i])
		slices.Reverse(seen)

		if len(seen) > len(unseen) { // Remove extra elements
			seen = seen[:len(unseen)]
		}
		if len(unseen) > len(seen) { // Remove extra elements
			unseen = unseen[:len(seen)]
		}

		// For part 1, only count as good the ones that are the same
		// For part 2, only count as good the ones that are not same, and differ in only in 1 element
		if diffs, areSame := areEqualSlices(seen, unseen); (!part2 && areSame) || (part2 && !areSame && diffs == 1) {
			return i
		}
	}

	return 0
}

// Rotate the matrix, so rows turn into cols, and cols into rows
func convertIntoCols(pattern [][]rune) [][]rune {
	patternReversed := make([][]rune, len(pattern[0]))
	for i := range patternReversed {
		patternReversed[i] = make([]rune, len(pattern))
	}

	for i, row := range pattern {
		for j := range row {
			patternReversed[j][i] = pattern[i][j]
		}
	}

	return patternReversed
}
