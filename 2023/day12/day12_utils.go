package day12

import (
	"strings"

	"golang.org/x/exp/slices"
)

func isValid(combination string, operationalRules []int) bool {
	groups := strings.Split(combination, ".")
	groups = slices.DeleteFunc(groups, func(e string) bool {
		return e == ""
	})

	if len(groups) != len(operationalRules) {
		return false
	}

	for i, group := range groups {
		if len(group) != operationalRules[i] {
			return false
		}
	}

	return true
}

func possibleArrangmentAmount(todo, partialArrangment string, grouping []int) int {
	matches := 0
	if len(todo) == 0 {
		if isValid(partialArrangment, grouping) {
			return 1
		}
		return 0
	}

	firstElement := string(todo[0])
	inputWithoutFirstElement := todo[1:]

	if firstElement == "?" {
		matches += possibleArrangmentAmount(inputWithoutFirstElement, partialArrangment+"#", grouping)
		matches += possibleArrangmentAmount(inputWithoutFirstElement, partialArrangment+".", grouping)
	} else {
		matches += possibleArrangmentAmount(inputWithoutFirstElement, partialArrangment+firstElement, grouping)
	}

	return matches
}
