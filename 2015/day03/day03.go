package day03

import (
	"fmt"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	return lines[0]
}

func (d Day) SolvePart1(instructionsI interface{}) (houses int) {
	instructions := instructionsI.(string)

	coord := Coordinate{X: 0, Y: 0}

	visits := make(map[string]int)
	visits["0.0"] = 1

	for _, d := range instructions {
		switch d {
		case '^':
			coord.Y--
		case 'v':
			coord.Y++
		case '<':
			coord.X--
		case '>':
			coord.X++
		}

		key := fmt.Sprintf("%d.%d", coord.X, coord.Y)

		if _, ok := visits[key]; ok {
			visits[key]++
		} else {
			visits[key] = 1
		}
	}

	for _, times := range visits {
		if times >= 1 {
			houses++
		}
	}

	return
}

type Coordinate struct {
	X int
	Y int
}

func (d Day) SolvePart2(instructionsI interface{}) (houses int) {
	instructions := instructionsI.(string)

	turn := 0

	turns := [2]Coordinate{{X: 0, Y: 0}, {X: 0, Y: 0}}

	visits := make(map[string]int)
	visits["0.0"] = 2

	for _, d := range instructions {
		switch d {
		case '^':
			turns[turn].Y--
		case 'v':
			turns[turn].Y++
		case '<':
			turns[turn].X--
		case '>':
			turns[turn].X++
		}

		key := fmt.Sprintf("%d.%d", turns[turn].X, turns[turn].Y)
		if turn == 0 {
			turn = 1
		} else {
			turn = 0
		}

		if _, ok := visits[key]; ok {
			visits[key]++
		} else {
			visits[key] = 1
		}
	}

	for _, times := range visits {
		if times >= 1 {
			houses++
		}
	}

	return
}
