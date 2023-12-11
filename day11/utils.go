package day11

import "math"

func getUniverseExpansions(universe []string) ([]int, []int) {
	expansionRows := make([]int, 0)
	colsWithGalaxies := make(map[int]bool)
	for i, row := range universe {
		hasGalaxy := false
		for j, elem := range row {
			if elem == '#' {
				hasGalaxy = true
				colsWithGalaxies[j] = true
			}
		}
		if !hasGalaxy {
			expansionRows = append(expansionRows, i)
		}
	}

	expansionCols := []int{}
	for i := 0; i < len(universe); i++ {
		if !colsWithGalaxies[i] {
			expansionCols = append(expansionCols, i)
		}
	}

	return expansionRows, expansionCols
}

func timesPathGoesThrough(expansions []int, oIndex, dIndex int) (passing int) {
	for _, expansion := range expansions {
		if (expansion > oIndex && expansion < dIndex) || (expansion < oIndex && expansion > dIndex) {
			passing++
		}
	}

	return
}

func calcDistance(origin, dest Coordinate, expansionCols, expansionRows []int, expansionAmount int) int {
	result := int(math.Abs(float64(dest.J-origin.J)) + math.Abs(float64(dest.I-origin.I))) // Manhattan distance

	passingRows := timesPathGoesThrough(expansionRows, origin.I, dest.I)
	passingCols := timesPathGoesThrough(expansionCols, origin.J, dest.J)

	result += passingRows * (expansionAmount - 1)
	result += passingCols * (expansionAmount - 1)

	return result
}

func getGalaxies(universe []string) (galaxies []Coordinate) {
	for i, row := range universe {
		for j, node := range row {
			if node == '#' {
				galaxies = append(galaxies, Coordinate{I: i, J: j})
			}
		}
	}

	return
}
