package day16

type Direction int

const (
	UP    Direction = 0
	DOWN  Direction = 1
	LEFT  Direction = 2
	RIGHT Direction = 3
)

type Light struct {
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

func moveLight(contraption [][]rune, energized map[Coordinate]bool, lights []Light, visited map[Visit]bool) int {
	energizedAmount := 0
	if len(lights) == 0 {
		return energizedAmount
	}

	newLights := []Light{}
	for _, light := range lights {
		if _, ok := energized[light.Coord]; !ok {
			energized[light.Coord] = true
			energizedAmount++
		}

		nextStates := nextState(light, contraption)

		if len(nextStates) > 0 {
			newLights = append(newLights, nextStates...)
		}
	}

	unvisitedsRemaining := false
	for _, newLight := range newLights {
		visit := Visit{Coord: newLight.Coord, Dir: newLight.Dir}
		if _, ok := visited[visit]; !ok {
			visited[visit] = true
			unvisitedsRemaining = true
		}
	}

	if unvisitedsRemaining {
		energizedAmount += moveLight(contraption, energized, newLights, visited)
	}

	return energizedAmount
}

func nextState(light Light, contraption [][]rune) []Light {
	i, j, dir := light.Coord.I, light.Coord.J, light.Dir

	newDir := dir

	newLights := []Light{}

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
			newLights = append(newLights,
				Light{Coord: Coordinate{I: i - 1, J: j}, Dir: UP},
				Light{Coord: Coordinate{I: i + 1, J: j}, Dir: DOWN})
		}
	case '-':
		if dir == UP || dir == DOWN {
			newLights = append(newLights,
				Light{Coord: Coordinate{I: i, J: j - 1}, Dir: LEFT},
				Light{Coord: Coordinate{I: i, J: j + 1}, Dir: RIGHT})
		}
	}

	if len(newLights) == 0 { // Element was not | nor -
		newI, newJ := nextPosition(i, j, newDir)
		if newI < 0 || newI > len(contraption)-1 || newJ < 0 || newJ > len(contraption[0])-1 {
			return []Light{}
		}
		return []Light{{Coord: Coordinate{I: newI, J: newJ}, Dir: newDir}}
	} else {
		validLights := []Light{}
		for _, newLight := range newLights {
			if newLight.Coord.I < 0 || newLight.Coord.I > len(contraption)-1 || newLight.Coord.J < 0 || newLight.Coord.J > len(contraption[0])-1 {
				continue
			}
			validLights = append(validLights, newLight)
		}

		return validLights
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
