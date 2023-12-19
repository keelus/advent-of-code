package day19

import (
	"log"
)

type Category rune

type Operation byte

type Part struct {
	Values map[Category]int
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

func (wf Workflow) parsePart(wfs map[string]Workflow, part Part) bool {
	for _, rule := range wf.Rules {
		if (rule.Op == '<' && part.Values[rule.Cat] < rule.Val) || (rule.Op == '>' && part.Values[rule.Cat] > rule.Val) {
			switch rule.Destination {
			case "A", "R":
				return rule.Destination == "A"
			default:
				return wfs[rule.Destination].parsePart(wfs, part)
			}
		}
	}

	switch wf.Exception {
	case "A", "R":
		return wf.Exception == "A"
	default:
		return wfs[wf.Exception].parsePart(wfs, part)
	}
}

func (wf Workflow) calcCombinations(wfs map[string]Workflow, ranges map[Category]Range) int {
	combinations := 0

	currentRanges := cloneRanges(ranges)

	for _, rule := range wf.Rules {
		cat := rule.Cat

		newSelfRange, adjacentRange := currentRanges[cat].resize(rule.Val, rule.Op)

		currentRanges[cat] = adjacentRange

		selfRanges := cloneRanges(currentRanges)
		selfRanges[cat] = newSelfRange

		if rule.Destination == "A" || rule.Destination == "R" {
			if rule.Destination == "A" {
				combinations += rangeCombinations(selfRanges)
			}
			continue
		}

		combinations += wfs[rule.Destination].calcCombinations(wfs, selfRanges)
	}

	if wf.Exception == "A" || wf.Exception == "R" {
		if wf.Exception == "A" {
			combinations += rangeCombinations(currentRanges)
		}
		return combinations
	}

	return combinations + wfs[wf.Exception].calcCombinations(wfs, currentRanges)
}

func cloneRanges(ranges map[Category]Range) map[Category]Range {
	newRanges := make(map[Category]Range, 4)

	for category, r := range ranges {
		newRanges[category] = Range{Min: r.Min, Max: r.Max}
	}

	return newRanges
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
