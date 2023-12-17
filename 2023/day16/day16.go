package day16

import (
	"sync"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	contraption := make([][]rune, len(lines))
	for i, line := range lines {
		contraption[i] = make([]rune, len(line))
		for j, elem := range line {
			contraption[i][j] = elem
		}
	}

	return contraption
}

func (d Day) SolvePart1(contraptionI interface{}) (energizedAmount int) {
	contraption := contraptionI.([][]rune)

	energizedAmount = moveBeam(contraption, Beam{Coord: Coordinate{I: 0, J: 0}, Dir: RIGHT})

	return energizedAmount
}

func (d Day) SolvePart2(contraptionI interface{}) int {
	contraption := contraptionI.([][]rune)

	var wg sync.WaitGroup
	var mu sync.Mutex

	var maxEnergizedAmount = 0

	// 0-n Rows
	origins := [2]int{0, len(contraption[0]) - 1}
	directions := [2]Direction{RIGHT, LEFT}
	for i := 0; i < 2; i++ {
		for row := 0; row < len(contraption); row++ {
			wg.Add(1)
			go func(rowIndex, colOrigin int, dir Direction) {
				defer wg.Done()
				energizedAmount := moveBeam(contraption, Beam{Coord: Coordinate{I: rowIndex, J: colOrigin}, Dir: dir})
				mu.Lock()
				if energizedAmount > maxEnergizedAmount {
					maxEnergizedAmount = energizedAmount
				}
				mu.Unlock()
			}(row, origins[i], directions[i])
		}
	}

	wg.Wait()

	// 0-m Columns
	origins = [2]int{0, len(contraption) - 1}
	directions = [2]Direction{DOWN, UP}
	for i := 0; i < 2; i++ {
		for col := 0; col < len(contraption[0]); col++ {
			wg.Add(1)
			go func(colIndex, rowOrigin int, dir Direction) {
				defer wg.Done()
				energizedAmount := moveBeam(contraption, Beam{Coord: Coordinate{I: rowOrigin, J: colIndex}, Dir: dir})
				mu.Lock()
				if energizedAmount > maxEnergizedAmount {
					maxEnergizedAmount = energizedAmount
				}
				mu.Unlock()
			}(col, origins[i], directions[i])
		}
	}

	wg.Wait()

	return maxEnergizedAmount
}
