package day15

import (
	"log"
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

type Day struct{}

type Step struct {
	Label  string
	Adding bool
	Lens   int
}

type Slot struct {
	Label string
	Lens  int
}

type ParsedInput struct {
	Words []string // Part 1
	Steps []Step   // Part 2
}

func (d Day) GetInput(lines []string) interface{} {
	words := strings.Split(lines[0], ",")
	steps := make([]Step, len(words))

	for i, word := range words {
		if strings.Contains(word, "=") {
			parts := strings.FieldsFunc(word, func(r rune) bool {
				return r == '='
			})
			number, err := strconv.Atoi(parts[1])
			if err != nil {
				log.Fatalf("Error while parsing the integer '%s'", parts[1])
			}
			steps[i] = Step{Label: parts[0], Adding: true, Lens: number}
		} else {
			steps[i] = Step{Label: word[:len(word)-1], Adding: false}
		}
	}

	return ParsedInput{Words: words, Steps: steps}
}

func (d Day) SolvePart1(parsedInputI interface{}) (hashSum int) {
	words := parsedInputI.(ParsedInput).Words
	for _, word := range words {
		hashSum += getWordHash(word)
	}
	return
}

func (d Day) SolvePart2(parsedInputI interface{}) (hashSum int) {
	steps := parsedInputI.(ParsedInput).Steps

	boxMap := make(map[int][]Slot, 0)

	for _, step := range steps {
		box := getWordHash(step.Label)

		if step.Adding {
			slotIndex := slices.IndexFunc(boxMap[box], func(e Slot) bool {
				return e.Label == step.Label
			})

			if slotIndex > -1 {
				boxMap[box][slotIndex].Lens = step.Lens
			} else {
				boxMap[box] = append(boxMap[box], Slot{Label: step.Label, Lens: step.Lens})
			}
		} else {
			boxMap[box] = slices.DeleteFunc(boxMap[box], func(e Slot) bool {
				return e.Label == step.Label
			})
		}
	}

	for i, box := range boxMap {
		for j, slot := range box {
			hashSum += (i + 1) * (j + 1) * slot.Lens
		}
	}

	return
}
