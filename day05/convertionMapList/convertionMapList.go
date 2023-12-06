package convertionMapList

import (
	"advent-of-code-23/day05/convertionMap"
)

type ConvertionMapList struct {
	List []convertionMap.ConvertionMap
}

func (cl *ConvertionMapList) GetNumberMapped(number int) int {
	for _, convertionMap := range cl.List {
		newValue, isMapped := convertionMap.GetNumberMapped(number)
		if isMapped {
			return newValue
		}
	}

	return number
}
