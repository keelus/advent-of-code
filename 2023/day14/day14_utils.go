package day14

import "golang.org/x/exp/slices"

type Direction int

const (
	NORTH Direction = iota
	WEST  Direction = 1
	SOUTH Direction = 2
	EAST  Direction = 3
)

func isEmpty(platform [][]rune, i, j int, dir Direction) bool {
	switch dir {
	case NORTH:
		if i < 0 {
			return false
		}
	case WEST:
		if j < 0 {
			return false
		}
	case SOUTH:
		if i == len(platform) {
			return false
		}
	case EAST:
		if j == len(platform[i]) {
			return false
		}
	}

	return platform[i][j] == '.'
}

func tilt(platform [][]rune, dir Direction) {
	switch dir {
	case NORTH:
		for i := 1; i < len(platform); i++ {
			for j := 0; j < len(platform[i]); j++ {
				if platform[i][j] == 'O' {
					placesToMove := 1
					for isEmpty(platform, i-placesToMove, j, NORTH) {
						placesToMove++
					}
					placesToMove-- // It means the previous could not be placed
					platform[i][j], platform[i-placesToMove][j] = platform[i-placesToMove][j], platform[i][j]
				}
			}
		}
	case WEST:
		for i := 0; i < len(platform); i++ {
			for j := 1; j < len(platform[i]); j++ {
				if platform[i][j] == 'O' {
					placesToMove := 1
					for isEmpty(platform, i, j-placesToMove, WEST) {
						placesToMove++
					}
					placesToMove--
					platform[i][j], platform[i][j-placesToMove] = platform[i][j-placesToMove], platform[i][j]
				}
			}
		}
	case SOUTH:
		for i := len(platform) - 2; i >= 0; i-- {
			for j := 0; j < len(platform[i]); j++ {
				if platform[i][j] == 'O' {
					placesToMove := 1
					for isEmpty(platform, i+placesToMove, j, SOUTH) {
						placesToMove++
					}
					placesToMove--
					platform[i][j], platform[i+placesToMove][j] = platform[i+placesToMove][j], platform[i][j]
				}
			}
		}
	case EAST:
		for i := 0; i < len(platform); i++ {
			for j := len(platform[i]) - 2; j >= 0; j-- {
				if platform[i][j] == 'O' {
					placesToMove := 1
					for isEmpty(platform, i, j+placesToMove, EAST) {
						placesToMove++
					}
					placesToMove--
					platform[i][j], platform[i][j+placesToMove] = platform[i][j+placesToMove], platform[i][j]
				}
			}
		}
	}
}

func calcLineLoad(line []rune, lineNum int) (load int) {
	for _, elem := range line {
		if elem == 'O' {
			load += lineNum
		}
	}

	return
}

func deepCopy(platform [][]rune) (platformCopy [][]rune) {
	platformCopy = make([][]rune, len(platform))

	for i, line := range platform {
		platformCopy[i] = make([]rune, len(line))
		copy(platformCopy[i], line)
	}

	return
}

// For part 2:
func hasBeenSeen(seenPlatforms [][][]rune, platform [][]rune) (int, bool) {
	for i, seenPlatform := range seenPlatforms {
		hasFullyMatched := true
		for j, line := range seenPlatform {
			if !slices.Equal(line, platform[j]) {
				hasFullyMatched = false
				break
			}
		}

		if hasFullyMatched {
			return i, true
		}
	}

	return -1, false
}
