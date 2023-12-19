package day19

import (
	"log"
	"strconv"
	"strings"
)

type Day struct{}

type ParsedInput struct {
	Workflows map[string]Workflow
	Parts     []Part
}

func (d Day) GetInput(lines []string) interface{} {
	workflows := make(map[string]Workflow)
	parts := make([]Part, 0)

	parsingParts := false
	for _, line := range lines {
		if !parsingParts {
			if line == "" {
				parsingParts = true
				continue
			}

			lineParts := strings.Split(strings.Replace(line, "}", "", 1), "{")
			rules := strings.Split(lineParts[1], ",")
			workflow := Workflow{
				Name:  lineParts[0],
				Rules: make([]Rule, len(rules)-1),
			}

			for i, sRule := range rules {
				if strings.ContainsAny(sRule, "<>") {
					lastParts := strings.Split(sRule[2:], ":")
					sValue := lastParts[0]

					value, err := strconv.Atoi(sValue)
					if err != nil {
						log.Fatalf("Error while parsing the integer '%s'", sValue)
					}

					workflow.Rules[i] = Rule{
						Cat:         Category(sRule[0]),
						Op:          Operation(sRule[1]),
						Val:         value,
						Destination: lastParts[1],
					}
				} else { // Is the exception
					workflow.Exception = sRule
				}
			}
			workflows[workflow.Name] = workflow
		} else {
			categoryAndValues := strings.Split(line[1:len(line)-1], ",")

			part := Part{Values: make(map[Category]int, 4)}

			for _, catAndVal := range categoryAndValues {
				sParts := strings.Split(catAndVal, "=")

				sValue := sParts[1]

				value, err := strconv.Atoi(sValue)
				if err != nil {
					log.Fatalf("Error while parsing the integer '%s'", sValue)
				}

				part.Values[Category(sParts[0][0])] = value
			}
			parts = append(parts, part)
		}
	}

	workflows["A"] = Workflow{Name: "A"}
	workflows["R"] = Workflow{Name: "R"}

	return ParsedInput{Workflows: workflows, Parts: parts}
}

func (d Day) SolvePart1(parsedInputI interface{}) (finalSum int) {
	parsedInput := parsedInputI.(ParsedInput)
	workflows := parsedInput.Workflows

	for _, part := range parsedInput.Parts {
		if workflows["in"].parsePart(workflows, part) {
			finalSum += part.sum()
		}
	}

	return
}

func (d Day) SolvePart2(parsedInputI interface{}) int {
	parsedInput := parsedInputI.(ParsedInput)
	workflows := parsedInput.Workflows

	ranges := map[Category]Range{
		'x': {Min: 1, Max: 4000},
		'm': {Min: 1, Max: 4000},
		'a': {Min: 1, Max: 4000},
		's': {Min: 1, Max: 4000},
	}

	return workflows["in"].getCombinations(workflows, ranges)
}
