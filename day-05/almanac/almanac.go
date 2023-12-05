package almanac

import (
	"advent-of-code-23/day-05/convertionMapList"

	"golang.org/x/exp/slices"
)

type Almanac struct {
	Seeds          []int
	ConvertionMaps [7]convertionMapList.ConvertionMapList
}

func (a *Almanac) GetLowestNearestLocation() int {
	mappingNumbers := a.Seeds

	for _, convertionMapList := range a.ConvertionMaps {
		for i, number := range mappingNumbers {
			value := convertionMapList.GetNumberMapped(number)
			mappingNumbers[i] = value
		}
	}

	return slices.Min(mappingNumbers)
}
