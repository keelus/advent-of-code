package day11

import (
	"golang.org/x/exp/slices"
)

type Day struct{}

type Node struct {
	Coord             Coordinate
	i, Cost           int
	IsGalaxy, Visited bool
	Previous          *Node
}

type Coordinate struct {
	I, J int
}

type ParsedInput struct {
	Universe                                 [][]*Node
	ColsWithoutGalaxies, RowsWithoutGalaxies []int
}

func (d Day) GetInput(lines []string) interface{} {
	universe := make([][]*Node, len(lines))
	for i, row := range lines {
		universe[i] = make([]*Node, len(row))
		for j, elem := range row {
			newNode := Node{Coord: Coordinate{I: i, J: j}, IsGalaxy: elem == '#', Cost: 1}
			universe[i][j] = &newNode
		}
	}

	rowsWithoutGalaxies, colsWithoutGalaxies := getColsAndRowsWithoutGalaxies(universe)

	return ParsedInput{Universe: universe, ColsWithoutGalaxies: colsWithoutGalaxies, RowsWithoutGalaxies: rowsWithoutGalaxies}
}

func getColsAndRowsWithoutGalaxies(universe [][]*Node) ([]int, []int) {
	rowsWithoutGalaxies := make([]int, 0)
	colsWithGalaxies := make(map[int]bool)
	for i, row := range universe {
		hasGalaxy := false
		for j, elem := range row {
			if elem.IsGalaxy {
				hasGalaxy = true
				colsWithGalaxies[j] = true
			}
		}
		if !hasGalaxy {
			rowsWithoutGalaxies = append(rowsWithoutGalaxies, i)
		}
	}

	colsWithoutGalaxies := []int{}
	for i := 0; i < len(universe); i++ {
		if !colsWithGalaxies[i] {
			colsWithoutGalaxies = append(colsWithoutGalaxies, i)
		}
	}

	return rowsWithoutGalaxies, colsWithoutGalaxies
}

func expandUniverse(universe [][]*Node, colsWithoutGalaxies, rowsWithoutGalaxies []int, expansionAmount int) [][]*Node {
	for i := 0; i < len(universe); i++ {
		for j := 0; j < len(universe[0]); j++ {
			universe[i][j].Cost = 0

			extendsCol := slices.Contains(colsWithoutGalaxies, j)
			extendsRow := slices.Contains(rowsWithoutGalaxies, i)

			if extendsCol {
				universe[i][j].Cost += expansionAmount
			}
			if extendsRow {
				universe[i][j].Cost += expansionAmount
			}

			if !extendsCol && !extendsRow {
				universe[i][j].Cost = 1
			}
		}
	}

	return universe
}

// https://en.wikipedia.org/wiki/Lee_algorithm
func shortestPath(universe [][]*Node, origin, dest Coordinate) int {
	y, x := origin.I, origin.J
	dy, dx := dest.I, dest.J

	for i, row := range universe {
		for j := range row {
			universe[i][j].i = -1
			universe[i][j].Visited = false
			universe[i][j].Previous = nil
			if i == y && j == x {
				universe[i][j].i = 0
				universe[i][j].Visited = true
			}
		}
	}

	current_i := 0
	for {
		current_i++
		for i, row := range universe {
			for j := range row {
				if universe[i][j].i == current_i-1 {
					neighbors := []*Node{}

					if i > 0 { // TOP NEIGHBOR
						if !universe[i-1][j].Visited {
							neighbors = append(neighbors, universe[i-1][j])
						}
					}
					if j < len(universe[0])-1 { // RIGHT NEIGHBOR
						if !universe[i][j+1].Visited {
							neighbors = append(neighbors, universe[i][j+1])
						}
					}
					if i < len(universe)-1 { //BOTTOM NEIGHBOR
						if !universe[i+1][j].Visited {
							neighbors = append(neighbors, universe[i+1][j])
						}
					}
					if j > 0 { // LEFT NEIGHBOR
						if !universe[i][j-1].Visited {
							neighbors = append(neighbors, universe[i][j-1])
						}
					}

					for k := range neighbors {
						neighbors[k].i = current_i
						neighbors[k].Visited = true

						neighbors[k].Previous = universe[i][j]

						if neighbors[k].Coord.I == dy && neighbors[k].Coord.J == dx {
							steps := 0
							currentNode := universe[dy][dx]
							path := []Coordinate{}

							for currentNode.Previous != nil {
								path = append(path, currentNode.Coord)
								currentNode = currentNode.Previous
							}

							for _, point := range path {
								steps += universe[point.I][point.J].Cost
							}

							return steps
						}
					}
				}
			}
		}
	}
}

func getGalaxies(universe [][]*Node) (galaxies []Coordinate) {
	for _, row := range universe {
		for _, node := range row {
			if node.IsGalaxy {
				galaxies = append(galaxies, node.Coord)
			}
		}
	}
	return
}

func (d Day) SolvePart1(parsedInputI interface{}) int {
	parsedInput := parsedInputI.(ParsedInput)
	universe := parsedInput.Universe

	expandUniverse(universe, parsedInput.ColsWithoutGalaxies, parsedInput.RowsWithoutGalaxies, 2)

	galaxies := getGalaxies(universe)

	steps := 0
	for i := 0; i < len(galaxies); i++ {
		for j := i + 1; j < len(galaxies); j++ {
			steps += shortestPath(universe, galaxies[i], galaxies[j])
		}
	}

	return steps
}

func (d Day) SolvePart2(parsedInputI interface{}) int {
	parsedInput := parsedInputI.(ParsedInput)
	universe := parsedInput.Universe

	expandUniverse(universe, parsedInput.ColsWithoutGalaxies, parsedInput.RowsWithoutGalaxies, 10)

	galaxies := getGalaxies(universe)

	steps := 0
	for i := 0; i < len(galaxies); i++ {
		for j := i + 1; j < len(galaxies); j++ {
			steps += shortestPath(universe, galaxies[i], galaxies[j])
		}
	}

	return steps
}
