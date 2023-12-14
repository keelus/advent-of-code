package day14

import (
	"golang.org/x/exp/slices"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	platform := make([][]rune, len(lines))
	for i, line := range lines {
		platform[i] = make([]rune, len(line))
		for j, elem := range line {
			platform[i][j] = elem
		}
	}
	return platform
}

func (d Day) SolvePart1(platformI interface{}) (totalLoad int) {
	platform := deepCopy(platformI.([][]rune))

	tiltNorth(platform)
	for i, line := range platform {
		totalLoad += calcLineLoad(line, len(platform)-i)
	}

	return totalLoad
}

func deepCopy(platform [][]rune) [][]rune {
	platformCopy := make([][]rune, len(platform))
	for i, line := range platform {
		platformCopy[i] = make([]rune, len(line))
		for j, elem := range line {
			platformCopy[i][j] = elem
		}
	}

	return platformCopy
}

func hasBeenSeen(seens [][][]rune, searchingPlatform [][]rune) (int, bool) {
	for i, seenPlatform := range seens {
		hasFullMatched := true
		for j, line := range seenPlatform {
			if !slices.Equal(line, searchingPlatform[j]) {
				hasFullMatched = false
				break
			}
		}

		if hasFullMatched {
			return i, true
		}

	}

	return -1, false
}

func (d Day) SolvePart2(platformI interface{}) (totalLoad int) {
	platform := platformI.([][]rune)

	seenPlatforms := [][][]rune{deepCopy(platform)}

	// We need to find a pattern between the cycles, as 1 billion is too big to compute
	iter := 0
	firstSeen := -1
	for {
		iter++

		tiltNorth(platform)
		tiltWest(platform)
		tiltSouth(platform)
		tiltEast(platform)

		var seen bool
		firstSeen, seen = hasBeenSeen(seenPlatforms, platform)
		if seen {
			break
		}

		seenPlatforms = append(seenPlatforms, deepCopy(platform))
	}

	repeatingCycles := seenPlatforms[firstSeen:]                           // Remove platforms that are not in the repeating pattern
	patternIndexAt1Bth := (1_000_000_000 - firstSeen) % (iter - firstSeen) // Translate the index from 1B to that Nth element in the known pattern
	patternAt1Bth := repeatingCycles[patternIndexAt1Bth]

	for i, line := range patternAt1Bth {
		totalLoad += calcLineLoad(line, len(platform)-i)
	}

	return
}
