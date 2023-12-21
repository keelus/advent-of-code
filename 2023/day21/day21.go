package day21

import (
	"image"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	garden := make([][]rune, len(lines))
	for i, line := range lines {
		garden[i] = make([]rune, len(line))
		for j, elem := range line {
			garden[i][j] = elem
		}
	}

	return garden
}

func (d Day) SolvePart1(gardenI interface{}) int {
	garden := gardenI.([][]rune)

	for step := 0; step < 64; step++ {
		newGarden := deep(garden)
		for i := range garden {
			for j := range garden[i] {
				if garden[i][j] != '#' {
					if reachable(garden, image.Point{j, i}) {
						newGarden[i][j] = 'O'
					} else {
						newGarden[i][j] = '.'
					}
				}
			}
		}
		garden = newGarden
	}

	totalReachable := 0
	for _, row := range garden {
		for _, elem := range row {
			if elem == 'O' || elem == 'S' {
				totalReachable++
			}
		}
	}

	return totalReachable
}

func (d Day) SolvePart2(gardenI interface{}) int {
	return -1
}
