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

func possibleArrangmentAmount(row Row) int {
	combinations := []string{""}

	for _, char := range row.Record {
		newCombinations := []string{}
		for _, existingCombination := range combinations {
			if char == '?' {
				newCombinations = append(newCombinations, existingCombination+"#", existingCombination+".")
			} else {
				newCombinations = append(newCombinations, existingCombination+string(char))
			}
		}
		combinations = newCombinations
	}

	validCombinationAmount := 0
	for _, combination := range combinations {
		if isValid(combination, row.OperationalGrouping) {
			validCombinationAmount++
		}
	}

	return validCombinationAmount
}
