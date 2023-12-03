package main

import (
	"advent-of-code-23/common"
	"fmt"
	"log"
	"strconv"
)

type Direction int

const (
	TOP Direction = iota
	RIGHT
	BOTTOM
	LEFT
)

type Part struct {
	Value        int
	SymbolCoords string
	Symbol       rune
}

func isSymbol(char rune) bool {
	return !((char >= '0' && char <= '9') || char == '.')
}

func isNumber(char rune) bool {
	return char >= '0' && char <= '9'
}

func getLineParts(line string, originalRow int, originalCol int, dir Direction, symbol rune) []Part {
	parts := make([]Part, 0)
	posBegin := -1
	posEnd := -1
	buffer := make([]rune, 0)

	symbolPos := originalCol
	if dir == LEFT {
		symbolPos--
	} else if dir == RIGHT {
		symbolPos++
	}

	for pos, char := range line {
		if isNumber(char) { // If is not a dot, we are reading a number
			if posBegin == -1 {
				posBegin = pos
			}
			buffer = append(buffer, char)
		}

		// If line end or a dot, we check the number if is not empty
		if (!isNumber(char) || pos == len(line)-1) && len(buffer) > 0 {
			posEnd = pos
			if !isNumber(char) {
				posEnd--
			}

			if dir == TOP || dir == BOTTOM {
				posBegin--
				posEnd++
			}

			if symbolPos >= posBegin && symbolPos <= posEnd {
				iPart, err := strconv.ParseInt(string(buffer), 10, 64)
				if err != nil {
					log.Fatalf("Error while parsing the part to integer '%s'", string(buffer))
				}
				part := Part{Value: int(iPart), SymbolCoords: fmt.Sprintf("%d.%d", originalCol, originalRow), Symbol: symbol}
				parts = append(parts, part)
			}

			// Clear the read number
			posBegin = -1
			posEnd = -1
			buffer = make([]rune, 0)
		}
	}

	return parts
}

func getPartAndGearSums(lines []string) (int, int) {
	foundParts := make([]Part, 0)
	for i, line := range lines {
		for j, char := range line {
			if isSymbol(char) {
				appendParts := func(direction Direction, lineIndex int) {
					for _, part := range getLineParts(lines[lineIndex], i, j, direction, char) {
						foundParts = append(foundParts, part)
					}
				}

				if i != 0 {
					appendParts(TOP, i-1)
				}

				if j != 0 {
					appendParts(LEFT, i)
				}

				if j != len(line)-1 {
					appendParts(RIGHT, i)
				}

				if i != len(lines)-1 {
					appendParts(BOTTOM, i+1)
				}
			}
		}
	}

	sum := 0
	gearSum := 0

	asteriskAdjAmount := make(map[string]int, 0)
	for _, part := range foundParts {
		sum += part.Value

		if part.Symbol == '*' {
			if _, ok := asteriskAdjAmount[part.SymbolCoords]; ok {
				asteriskAdjAmount[part.SymbolCoords]++
			} else {
				asteriskAdjAmount[part.SymbolCoords] = 1
			}
		}
	}

	for symbolCoords, adjAmount := range asteriskAdjAmount {
		if adjAmount == 2 {
			ratio := 1
			for _, part := range foundParts {
				if part.SymbolCoords == symbolCoords {
					ratio *= part.Value
				}
			}
			gearSum += ratio
		}
	}

	return sum, gearSum
}

func main() {
	lines := common.GetInputByLines("")

	sum, gearSum := getPartAndGearSums(lines)

	log.Printf("[Part 1] Part sum: %d\n", sum)
	log.Printf("[Part 2] Gear ratio sum: %d\n", gearSum)

}
