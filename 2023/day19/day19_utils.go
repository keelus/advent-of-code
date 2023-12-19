package day19

import (
	"log"

	"golang.org/x/exp/maps"
)

type Category rune

type Operation byte

type Part struct {
	Values map[Category]int
}

func (p Part) sum() (sumVal int) {
	for _, val := range p.Values {
		sumVal += val
	}
	return
}

type Range struct {
	Min int
	Max int
}

type Workflow struct {
	Name      string
	Rules     []Rule
	Exception string
}

type Rule struct {
	Cat         Category
	Op          Operation
	Val         int
	Destination string
}

func (workflow Workflow) parsePart(workflows map[string]Workflow, part Part) bool {
	switch workflow.Name {
	case "A", "R":
		return workflow.Name == "A"
	default:
		for _, rule := range workflow.Rules {
			if (rule.Op == '<' && part.Values[rule.Cat] < rule.Val) || (rule.Op == '>' && part.Values[rule.Cat] > rule.Val) {
				return workflows[rule.Destination].parsePart(workflows, part)
			}
		}

		return workflows[workflow.Exception].parsePart(workflows, part)
	}
}

func (workflow Workflow) getCombinations(workflows map[string]Workflow, ranges map[Category]Range) int {
	switch workflow.Name {
	case "A":
		return rangeCombinations(ranges)
	case "R":
		return 0
	default:
		combinations := 0
		currentRanges := maps.Clone(ranges)

		for _, rule := range workflow.Rules {
			newSelfRange, adjacentRange := currentRanges[rule.Cat].resize(rule.Val, rule.Op)

			currentRanges[rule.Cat] = adjacentRange

			selfRanges := maps.Clone(currentRanges)
			selfRanges[rule.Cat] = newSelfRange

			combinations += workflows[rule.Destination].getCombinations(workflows, selfRanges)
		}

		return combinations + workflows[workflow.Exception].getCombinations(workflows, currentRanges)
	}
}

func (r Range) resize(value int, operation Operation) (Range, Range) {
	selfRange := Range{}
	adjacentRange := Range{}
	switch operation {
	case '<':
		selfRange.Min = r.Min
		selfRange.Max = value - 1

		adjacentRange.Min = value
		adjacentRange.Max = r.Max
	case '>':
		selfRange.Min = value + 1
		selfRange.Max = r.Max

		adjacentRange.Min = r.Min
		adjacentRange.Max = value
	default:
		log.Fatalf("Unexpected operation")
	}

	return selfRange, adjacentRange
}

func rangeCombinations(ranges map[Category]Range) int {
	combinations := 1
	for _, val := range ranges {
		combinations *= val.Max - val.Min + 1
	}
	return combinations
}
