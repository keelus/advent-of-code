package day18

import (
	"log"
	"strconv"
	"strings"
)

var BYTE_TO_INT = map[byte]byte{'R': '0', 'D': '1', 'L': '2', 'U': '3'}

type Day struct{}

type ParsedInput struct {
	InstructionsPart1 []Instruction
	InstructionsPart2 []Instruction
}

func (d Day) GetInput(lines []string) interface{} {
	instructionsPart1 := make([]Instruction, len(lines))
	instructionsPart2 := make([]Instruction, len(lines))
	for i, line := range lines {
		lineParts := strings.Fields(line)

		// Part 1
		amount, err := strconv.Atoi(lineParts[1])
		if err != nil {
			log.Fatalf("Error while parsing the integer '%s'", lineParts[1])
		}

		instructionsPart1[i] = Instruction{Dir: BYTE_TO_INT[lineParts[0][0]], Amount: amount}

		// Part 2
		sAmountHex := lineParts[2][2:7]

		amount2, err := strconv.ParseInt(sAmountHex, 16, 64)
		if err != nil {
			log.Fatalf("Error while parsing the hexadecimal to integer '%s'", sAmountHex)
		}

		instructionsPart2[i] = Instruction{Dir: lineParts[2][7], Amount: int(amount2)}
	}

	return ParsedInput{InstructionsPart1: instructionsPart1, InstructionsPart2: instructionsPart2}
}

func (d Day) SolvePart1(parsedInputI interface{}) int {
	instructions := parsedInputI.(ParsedInput).InstructionsPart1
	cornerPoints, perimeter := parseTerrain(instructions)

	return getTotalArea(cornerPoints, perimeter)
}

func (d Day) SolvePart2(parsedInputI interface{}) int {
	instructions := parsedInputI.(ParsedInput).InstructionsPart2
	cornerPoints, perimeter := parseTerrain(instructions)

	return getTotalArea(cornerPoints, perimeter)
}
