package day17

import (
	"image"
	"log"
	"strconv"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	island := make([][]int, len(lines))
	for i, line := range lines {
		island[i] = make([]int, len(line))
		for j, elem := range line {
			cost, err := strconv.Atoi(string(elem))
			if err != nil {
				log.Fatalf("Error while parsing the integet '%s'", string(elem))
			}

			island[i][j] = cost
		}
	}

	return island
}

func (d Day) SolvePart1(islandI interface{}) int {
	island := islandI.([][]int)

	return shortestPathCost(island, image.Point{0, 0}, image.Point{len(island) - 1, len(island[0]) - 1}, 1, 3)
}

func (d Day) SolvePart2(islandI interface{}) int {
	island := islandI.([][]int)

	return shortestPathCost(island, image.Point{0, 0}, image.Point{len(island) - 1, len(island[0]) - 1}, 4, 10)
}
