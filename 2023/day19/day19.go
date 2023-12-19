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
	workflows := map[string]Workflow{}
	parts := []Part{}

	// to-do parsing optimization
	parsingParts := false
	for _, line := range lines {
		if !parsingParts {
			if line == "" {
				parsingParts = true
				continue
			}

			workflow := Workflow{}
			baseParts := strings.Split(strings.Replace(line, "}", "", 1), "{")
			name := baseParts[0]

			sRules := strings.Split(baseParts[1], ",")
			rules := make([]Rule, len(sRules)-1)
			for i, sRule := range sRules {
				if strings.ContainsAny(sRule, "<>") {
					rCategory := sRule[0]
					rOperation := sRule[1]

					lastParts := strings.Split(sRule[2:], ":")
					sValue := lastParts[0]
					destination := lastParts[1]

					value, err := strconv.Atoi(sValue)
					if err != nil {
						log.Fatalf("Error while parsing the integer '%s'", sValue)
					}

					rules[i] = Rule{Cat: Category(rCategory), Op: Operation(rOperation), Val: value, Destination: destination}
				} else { // Is the exception
					workflow.Exception = sRule
				}
			}

			workflow.Rules = rules
			workflow.Name = name

			workflows[name] = workflow
		} else {
			categoryAndValues := strings.Split(line[1:len(line)-1], ",")

			part := Part{}
			part.Values = make(map[Category]int, 4)

			for _, catAndVal := range categoryAndValues {
				sParts := strings.Split(catAndVal, "=")

				rCategory := sParts[0][0]
				sValue := sParts[1]

				value, err := strconv.Atoi(sValue)
				if err != nil {
					log.Fatalf("Error while parsing the integer '%s'", sValue)
				}

				part.Values[Category(rCategory)] = value
			}
			parts = append(parts, part)
		}
	}

	return ParsedInput{Workflows: workflows, Parts: parts}
}

func (d Day) SolvePart1(parsedInputI interface{}) int {
	parsedInput := parsedInputI.(ParsedInput)

	workflows := parsedInput.Workflows

	finalSum := 0
	for _, part := range parsedInput.Parts {
		if workflows["in"].parsePart(workflows, part) {
			for _, val := range part.Values {
				finalSum += val
			}
		}
	}

	return finalSum
}

func (d Day) SolvePart2(parsedInputI interface{}) int {
	return -1
}
