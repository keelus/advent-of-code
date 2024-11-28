package day16

import (
	"2023/common/pair"
)

type Beam struct {
	Coord pair.Pair
	Dir   pair.Pair
}

type Visit struct {
	Coord pair.Pair
	Dir   pair.Pair
}

func moveBeam(contraption [][]rune, startBeam Beam) int {
	energized := make(map[pair.Pair]struct{})
	visited := make(map[Visit]struct{})

	energizedAmount := 0

	stack := []Beam{startBeam}

	for len(stack) > 0 {
		beam := stack[0]
		stack = stack[1:]

		if _, ok := energized[beam.Coord]; !ok {
			energized[beam.Coord] = struct{}{}
			energizedAmount++
		}

		newBeams := evaluate(beam, contraption)

		for _, newBeam := range newBeams {
			visit := Visit{Coord: newBeam.Coord, Dir: newBeam.Dir.Copy()}
			if _, ok := visited[visit]; !ok {
				visited[visit] = struct{}{}
				stack = append(stack, newBeam)
			}
		}
	}

	return energizedAmount
}

func evaluate(beam Beam, contraption [][]rune) []Beam {
	_, _, dir := beam.Coord.I, beam.Coord.J, beam.Dir.Copy()

	newDir := dir

	newBeams := []Beam{}

	switch contraption[beam.Coord.I][beam.Coord.J] {
	case '/', '\\':
		if dir.Perp(pair.New(1, 0)) {
			newDir = dir.TurnL()
		} else {
			newDir = dir.TurnR()
		}

		if contraption[beam.Coord.I][beam.Coord.J] == '\\' { // In \, we have to negate the new direction
			newDir = newDir.Neg()
		}
	case '|':
		if dir.Perp(pair.New(1, 0)) {
			newBeams = append(newBeams,
				Beam{Coord: beam.Coord.Add(pair.Up()), Dir: pair.Up()},
				Beam{Coord: beam.Coord.Add(pair.Down()), Dir: pair.Down()})
		}
	case '-':
		if dir.Perp(pair.New(0, 1)) {
			newBeams = append(newBeams,
				Beam{Coord: beam.Coord.Add(pair.Left()), Dir: pair.Left()},
				Beam{Coord: beam.Coord.Add(pair.Right()), Dir: pair.Right()})
		}
	}

	if len(newBeams) == 0 { // Element was not | nor -
		newPos := beam.Coord.Add(newDir)
		if newPos.InBounds(0, 0, len(contraption), len(contraption[0])) {
			return []Beam{{Coord: newPos, Dir: newDir}}
		}
		return []Beam{}
	} else {
		validBeams := []Beam{}
		for _, newBeam := range newBeams {
			if newBeam.Coord.InBounds(0, 0, len(contraption), len(contraption[0])) {
				validBeams = append(validBeams, newBeam)
			}
		}

		return validBeams
	}
}
