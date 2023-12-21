package day21

import (
	"image"
)

func reachable(garden [][]rune, point image.Point) bool {
	directions := []image.Point{{0, -1}, {0, 1}, {-1, 0}, {1, 0}}

	for _, dir := range directions {
		p := point.Add(dir)
		if p.X >= 0 && p.X < len(garden[0]) && p.Y >= 0 && p.Y < len(garden) {
			if garden[p.Y][p.X] == 'O' || garden[p.Y][p.X] == 'S' {
				return true
			}
		}
	}

	return false
}

func deep(garden [][]rune) [][]rune {
	newGarden := make([][]rune, len(garden))
	for i := range garden {
		newGarden[i] = make([]rune, len(garden[i]))
		for j := range garden[i] {
			newGarden[i][j] = garden[i][j]
		}
	}
	return newGarden
}
