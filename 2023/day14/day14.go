package day14

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

	tilt(platform, NORTH)
	for i, line := range platform {
		totalLoad += calcLineLoad(line, len(platform)-i)
	}

	return totalLoad
}

func (d Day) SolvePart2(platformI interface{}) (totalLoad int) {
	platform := platformI.([][]rune)

	seenPlatforms := [][][]rune{deepCopy(platform)}

	// We need to find a pattern between the cycles, as 1 billion is too big to compute
	ith := 0
	firstSeen := -1
	for {
		ith++

		tilt(platform, NORTH)
		tilt(platform, WEST)
		tilt(platform, SOUTH)
		tilt(platform, EAST)

		var seen bool
		if firstSeen, seen = hasBeenSeen(seenPlatforms, platform); seen {
			break
		}

		seenPlatforms = append(seenPlatforms, deepCopy(platform))
	}

	repeatingCycles := seenPlatforms[firstSeen:]                          // Remove platforms that are not in the repeating pattern
	patternIndexAt1Bth := (1_000_000_000 - firstSeen) % (ith - firstSeen) // Translate the index from 1B to that Nth element in the known pattern
	patternAt1Bth := repeatingCycles[patternIndexAt1Bth]

	for i, line := range patternAt1Bth {
		totalLoad += calcLineLoad(line, len(platform)-i)
	}

	return
}
