package day17

import (
	"log"
	"strconv"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	island := make([][]Node, len(lines))
	for i, line := range lines {
		island[i] = make([]Node, len(line))
		for j, elem := range line {
			cost, err := strconv.Atoi(string(elem))
			if err != nil {
				log.Fatalf("Error while parsing the integet '%s'", string(elem))
			}

			island[i][j] = Node{Coord: Coordinate{I: i, J: j}, Cost: cost}
		}
	}

	return island
}

func (d Day) SolvePart1(islandI interface{}) int {
	island := islandI.([][]Node)

	return shortestPathCost(island, Coordinate{I: 0, J: 0}, Coordinate{I: len(island) - 1, J: len(island[0]) - 1}, 1, 3)
}

func (d Day) SolvePart2(islandI interface{}) int {
	return -1
}
