package day05

import (
	"advent-of-code/2023/day05/almanac"
	"advent-of-code/2023/day05/convertionMap"
	"advent-of-code/2023/day05/convertionMapList"
	"log"
	"strconv"
	"strings"
)

type Day struct{}

func parseSeeds(line string) []int {
	sNumbers := strings.Split(strings.TrimSpace(strings.Split(line, ":")[1]), " ")
	seeds := make([]int, 0)
	for _, sNumber := range sNumbers {
		number, err := strconv.ParseInt(sNumber, 10, 64)
		if err != nil {
			log.Fatalf("Error while parsing the integet '%s'", sNumber)
		}
		seeds = append(seeds, int(number))
	}

	return seeds
}

func (d Day) GetInput(lines []string) interface{} {
	seeds := parseSeeds(lines[0])
	var convertionMaps [7]convertionMapList.ConvertionMapList

	readingMap := -1
	for i, line := range lines {
		if line == "" || i == 0 {
			continue
		}

		if strings.Contains(line, ":") {
			readingMap++
			convertionMaps[readingMap] = convertionMapList.ConvertionMapList{}
			continue
		}

		sNumbers := strings.Split(strings.TrimSpace(line), " ")
		numbers := make([]int, 0)
		for _, sNumber := range sNumbers {
			number, err := strconv.ParseInt(sNumber, 10, 64)
			if err != nil {
				log.Fatalf("Error while parsing the integet '%s'", sNumber)
			}
			numbers = append(numbers, int(number))
		}

		convertionMap := convertionMap.ConvertionMap{Source: numbers[1], Destination: numbers[0], Length: numbers[2]}
		convertionMaps[readingMap] = convertionMapList.ConvertionMapList{List: append(convertionMaps[readingMap].List, convertionMap)}
	}

	return almanac.Almanac{Seeds: seeds, ConvertionMaps: convertionMaps}
}

func (d Day) SolvePart1(almanacI interface{}) int {
	almanacPart := almanacI.(almanac.Almanac)
	return almanacPart.GetLowestNearestLocationPart1()
}

func (d Day) SolvePart2(almanacI interface{}) int {
	almanacPart := almanacI.(almanac.Almanac)
	return almanacPart.GetLowestNearestLocationPart2()
}
