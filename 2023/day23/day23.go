package day23

import (
	"2023/common/pair"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	grid := make(Grid, len(lines))
	for i, line := range lines {
		grid[i] = make([]Node, len(line))
		for j, elem := range line {
			forcesDir := pair.Pair{}
			forces := true

			switch lines[i][j] {
			case '^':
				forcesDir = pair.Up()
			case 'v':
				forcesDir = pair.Down()
			case '<':
				forcesDir = pair.Left()
			case '>':
				forcesDir = pair.Right()
			default:
				forces = false
			}

			grid[i][j] = Node{
				Coord:     pair.New(i, j),
				IsForest:  elem == '#',
				Forces:    forces,
				ForcesDir: forcesDir,
			}
		}
	}

	return grid
}

func (d Day) SolvePart1(gridI interface{}) int {
	grid := gridI.(Grid)
	return grid.explorePath(1, pair.New(0, 1), pair.Down(), pair.New(len(grid)-1, len(grid[0])-2))
}

func (d Day) SolvePart2(gridI interface{}) int {
	return -1
}
