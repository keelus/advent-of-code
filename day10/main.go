package day10

type Day struct{}

type Direction string

const (
	START_PIPE           = 'S'
	NODIR      Direction = "NODIR"
	UP         Direction = "UP"
	RIGHT      Direction = "RIGHT"
	DOWN       Direction = "DOWN"
	LEFT       Direction = "LEFT"
)

func (d Direction) Opposite() Direction {
	switch d {
	case UP:
		return DOWN
	case DOWN:
		return UP
	case LEFT:
		return RIGHT
	case RIGHT:
		return LEFT
	}
	return NODIR
}

var PIPEDIRS = map[rune]map[Direction]bool{
	'|': {
		UP:    true,
		RIGHT: false,
		DOWN:  true,
		LEFT:  false,
	},
	'-': {
		UP:    false,
		RIGHT: true,
		DOWN:  false,
		LEFT:  true,
	},
	'L': {
		UP:    true,
		RIGHT: true,
		DOWN:  false,
		LEFT:  false,
	},
	'J': {
		UP:    true,
		RIGHT: false,
		DOWN:  false,
		LEFT:  true,
	},
	'7': {
		UP:    false,
		RIGHT: false,
		DOWN:  true,
		LEFT:  true,
	},
	'F': {
		UP:    false,
		RIGHT: true,
		DOWN:  true,
		LEFT:  false,
	},
	'.': {
		UP:    false,
		RIGHT: false,
		DOWN:  false,
		LEFT:  false,
	},
	'S': {
		UP:    true,
		RIGHT: true,
		DOWN:  true,
		LEFT:  true,
	},
}

type Node struct {
	Next *Node
	Rune rune
	Pos  [2]int
}

// Priority -> Top, Right, Down, Left

func getNextRune(pipeMap [][]rune, tgtI, tgtJ int, previousInvertedDir Direction) ([2]int, Direction) {
	connectedWidth := getConnectedWith(pipeMap, tgtI, tgtJ)

	previousDir := previousInvertedDir.Opposite()

	if val, ok := connectedWidth[UP]; ok && previousDir != UP {
		return val, UP
	}
	if val, ok := connectedWidth[RIGHT]; ok && previousDir != RIGHT {
		return val, RIGHT
	}
	if val, ok := connectedWidth[DOWN]; ok && previousDir != DOWN {
		return val, DOWN
	}
	if val, ok := connectedWidth[LEFT]; ok && previousDir != LEFT {
		return val, LEFT
	}
	return [2]int{-1, -1}, NODIR
}

func getConnectedWith(pipeMap [][]rune, tgtI, tgtJ int) map[Direction][2]int {
	connectedWith := make(map[Direction][2]int, 4)
	// If top is connected with tgt (which is bottom):
	if tgtI > 0 && PIPEDIRS[pipeMap[tgtI][tgtJ]][UP] {
		if PIPEDIRS[pipeMap[tgtI-1][tgtJ]][DOWN] {
			connectedWith[UP] = [2]int{tgtI - 1, tgtJ}
		}
	}
	// If bottom is connected with tgt (which is top):
	if tgtI < len(pipeMap)-1 && PIPEDIRS[pipeMap[tgtI][tgtJ]][DOWN] {
		if PIPEDIRS[pipeMap[tgtI+1][tgtJ]][UP] {
			connectedWith[DOWN] = [2]int{tgtI + 1, tgtJ}
		}
	}
	// If left is connected with tgt (which is right):
	if tgtJ > 0 && PIPEDIRS[pipeMap[tgtI][tgtJ]][LEFT] {
		if PIPEDIRS[pipeMap[tgtI][tgtJ-1]][RIGHT] {
			connectedWith[LEFT] = [2]int{tgtI, tgtJ - 1}
		}
	}
	// If right is connected with tgt (which is left):
	if tgtJ < len(pipeMap[tgtI])-1 && PIPEDIRS[pipeMap[tgtI][tgtJ]][RIGHT] {
		if PIPEDIRS[pipeMap[tgtI][tgtJ+1]][LEFT] {
			connectedWith[RIGHT] = [2]int{tgtI, tgtJ + 1}
		}
	}

	return connectedWith
}

func (d Day) GetInput(lines []string) interface{} {
	pipeMap := make([][]rune, len(lines))

	startPos := [2]int{-1, -1}
	for i, line := range lines {
		pipeMap[i] = make([]rune, len(line))
		for j, element := range line {
			pipeMap[i][j] = element
			if element == START_PIPE {
				startPos = [2]int{i, j}
			}
		}
	}

	beginNode := &Node{Rune: START_PIPE, Next: nil, Pos: startPos}

	currentNode := beginNode
	lastUsedPos := NODIR
	for {
		var newPos [2]int
		newPos, lastUsedPos = getNextRune(pipeMap, currentNode.Pos[0], currentNode.Pos[1], lastUsedPos)

		if newPos == startPos {
			break
		}

		currentNode.Next = &Node{Rune: pipeMap[newPos[0]][newPos[1]], Pos: newPos, Next: nil}
		currentNode = currentNode.Next
	}
	currentNode.Next = beginNode // Close last node with starting node

	return beginNode
}

func (d Day) SolvePart1(beginNodeI interface{}) int {
	beginNode := beginNodeI.(*Node)

	currentNode := beginNode
	cicleSteps := 0 // Counting the S
	for !(currentNode == beginNode && cicleSteps > 0) {
		currentNode = currentNode.Next
		cicleSteps++
	}

	return cicleSteps / 2
}

func (d Day) SolvePart2(mapParseI interface{}) int {
	return -1
}
