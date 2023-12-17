package day16

type Direction uint8

const (
	UP    Direction = 0
	DOWN  Direction = 1
	LEFT  Direction = 2
	RIGHT Direction = 3
)

type Beam struct {
	Coord Coordinate
	Dir   Direction
}

type Coordinate struct {
	I int
	J int
}

type Visit struct {
	Coord Coordinate
	Dir   Direction
}

func moveBeam(contraption [][]rune, startBeam Beam) int {
	energized := make(map[Coordinate]struct{})
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
			visit := Visit{Coord: newBeam.Coord, Dir: newBeam.Dir}
			if _, ok := visited[visit]; !ok {
				visited[visit] = struct{}{}
				stack = append(stack, newBeam)
			}
		}
	}

	return energizedAmount
}

func evaluate(beam Beam, contraption [][]rune) []Beam {
	i, j, dir := beam.Coord.I, beam.Coord.J, beam.Dir

	newDir := dir

	newBeams := []Beam{}

	switch contraption[i][j] {
	case '/':
		switch dir {
		case UP:
			newDir = RIGHT
		case DOWN:
			newDir = LEFT
		case LEFT:
			newDir = DOWN
		case RIGHT:
			newDir = UP
		}
	case '\\':
		switch dir {
		case UP:
			newDir = LEFT
		case DOWN:
			newDir = RIGHT
		case LEFT:
			newDir = UP
		case RIGHT:
			newDir = DOWN
		}
	case '|':
		if dir == LEFT || dir == RIGHT {
			newBeams = append(newBeams,
				Beam{Coord: Coordinate{I: i - 1, J: j}, Dir: UP},
				Beam{Coord: Coordinate{I: i + 1, J: j}, Dir: DOWN})
		}
	case '-':
		if dir == UP || dir == DOWN {
			newBeams = append(newBeams,
				Beam{Coord: Coordinate{I: i, J: j - 1}, Dir: LEFT},
				Beam{Coord: Coordinate{I: i, J: j + 1}, Dir: RIGHT})
		}
	}

	if len(newBeams) == 0 { // Element was not | nor -
		newI, newJ := nextPosition(i, j, newDir)
		if newI < 0 || newI > len(contraption)-1 || newJ < 0 || newJ > len(contraption[0])-1 {
			return []Beam{}
		}
		return []Beam{{Coord: Coordinate{I: newI, J: newJ}, Dir: newDir}}
	} else {
		validBeams := []Beam{}
		for _, newBeam := range newBeams {
			if newBeam.Coord.I < 0 || newBeam.Coord.I > len(contraption)-1 || newBeam.Coord.J < 0 || newBeam.Coord.J > len(contraption[0])-1 {
				continue
			}
			validBeams = append(validBeams, newBeam)
		}

		return validBeams
	}
}

func nextPosition(i, j int, dir Direction) (int, int) {
	switch dir {
	case UP:
		return i - 1, j
	case DOWN:
		return i + 1, j
	case LEFT:
		return i, j - 1
	case RIGHT:
		return i, j + 1
	}

	return -1, -1
}
