package day16

import (
	"math"
)

type Direction string

const (
	UP    Direction = "UP"
	DOWN  Direction = "DOWN"
	LEFT  Direction = "LEFT"
	RIGHT Direction = "RIGHT"
)

type Light struct {
	I   int
	J   int
	Dir Direction
}

func moveLight(contraption, energized [][]rune, lights []Light, prevEnergized int, repetitionAcc int) {
	if len(lights) == 0 {
		return
	}

	newLights := []Light{}
	for _, light := range lights {
		energized[light.I][light.J] = '#'
		nextStates := nextState(light, contraption, energized)

		if len(nextStates) > 0 {
			newLights = append(newLights, nextStates...)
		}
	}

	if getEnergizedAmount(energized) == prevEnergized {
		repetitionAcc++

		max := int(math.Max(float64(len(contraption)), float64(len(contraption[0]))))
		if repetitionAcc > max {
			return
		}
	} else {
		repetitionAcc = 0
	}

	moveLight(contraption, energized, newLights, getEnergizedAmount(energized), repetitionAcc)

	return
}

func nextState(light Light, contraption, energized [][]rune) []Light {
	i := light.I
	j := light.J
	dir := light.Dir

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
			newLights = append(newLights, Light{I: i - 1, J: j, Dir: UP}, Light{I: i + 1, J: j, Dir: DOWN})
		}
	case '-':
		if dir == UP || dir == DOWN {
			newLights = append(newLights, Light{I: i, J: j - 1, Dir: LEFT}, Light{I: i, J: j + 1, Dir: RIGHT})
		}
	}

	if len(newLights) == 0 { // Element was not | nor -
		newI, newJ := nextPosition(i, j, newDir)
		if newI < 0 || newI > len(contraption)-1 || newJ < 0 || newJ > len(contraption[0])-1 {
			return []Light{}
		}
		return []Light{{I: newI, J: newJ, Dir: newDir}}
	} else {
		validLights := []Light{}
		for _, newLight := range newLights {
			if newLight.I < 0 || newLight.I > len(contraption)-1 || newLight.J < 0 || newLight.J > len(contraption[0])-1 {
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

func getEnergizedAmount(energized [][]rune) (amount int) {
	for _, row := range energized {
		for _, elem := range row {
			if elem == '#' {
				amount++
			}
		}
	}

	return
}
