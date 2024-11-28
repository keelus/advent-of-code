package almanac

import (
	"2023/day05/convertionMapList"
	"math"
	"sync"

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

	var smallest = math.MaxInt

	var wg sync.WaitGroup
	var mu sync.Mutex

	for i := 0; i < len(a.Seeds); i += 2 {
		wg.Add(1)
		beginning := a.Seeds[i]
		length := a.Seeds[i+1]

		go func(beginning int, length int) {
			defer wg.Done()
			curSmallest := math.MaxInt

			for j := beginning; j < beginning+length; j++ {
				value := j
				for k, convertionMapList := range a.ConvertionMaps {
					value = convertionMapList.GetNumberMapped(value)
					if k == 6 && value < curSmallest {
						curSmallest = value
					}
				}
			}

			mu.Lock()
			if curSmallest < smallest {
				smallest = curSmallest
			}
			mu.Unlock()
		}(beginning, length)
	}

	wg.Wait()

	return smallest
}
