package almanac

import (
	"advent-of-code-23/day05/convertionMapList"
	"math"

	"golang.org/x/exp/slices"
)

type Almanac struct {
	Seeds          []int
	ConvertionMaps [7]convertionMapList.ConvertionMapList
}

func (a *Almanac) GetLowestNearestLocationPart1() int {
	mappingNumbers := slices.Clone(a.Seeds)

	for _, convertionMapList := range a.ConvertionMaps {
		for i, number := range mappingNumbers {
			value := convertionMapList.GetNumberMapped(number)
			mappingNumbers[i] = value
		}
	}

	return slices.Min(mappingNumbers)
}

func (a *Almanac) GetLowestNearestLocationPart2() int {
	smallest := math.MaxInt

	for i := 0; i < len(a.Seeds); i += 2 {
		beginning := a.Seeds[i]
		length := a.Seeds[i+1]

		for j := beginning; j < beginning+length; j++ {
			value := j
			for k, convertionMapList := range a.ConvertionMaps {
				value = convertionMapList.GetNumberMapped(value)
				if k == 6 && value < smallest {
					smallest = value
				}
			}
		}
	}

	return smallest
}
