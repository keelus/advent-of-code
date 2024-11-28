package day11

import (
	"2023/common/pair"
)

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

func calcDistance(origin, dest pair.Pair, expansionCols, expansionRows []int, expansionAmount int) int {
	result := origin.MDist(dest)

	passingRows := timesPathGoesThrough(expansionRows, origin.I, dest.I)
	passingCols := timesPathGoesThrough(expansionCols, origin.J, dest.J)

	result += passingRows * (expansionAmount - 1)
	result += passingCols * (expansionAmount - 1)

	return result
}

func getGalaxies(universe []string) (galaxies []pair.Pair) {
	for i, row := range universe {
		for j, node := range row {
			if node == '#' {
				galaxies = append(galaxies, pair.New(i, j))
			}
		}
	}

	return
}
