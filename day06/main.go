package day06

import (
	"log"
	"strconv"
	"strings"
)

type Day struct{}

type ParseResult struct {
	Part1Parse [2][]int
	Part2Parse [2]int
}

func (d Day) GetInput(lines []string) interface{} {
	parseResult := ParseResult{}
	for i := 0; i < 2; i++ {
		lineSNumbers := strings.Fields(lines[i])[1:]

		// Part 1 parsing:
		for _, sNumber := range lineSNumbers {
			if sNumber == "" {
				continue
			}

			number, err := strconv.Atoi(sNumber)
			if err != nil {
				log.Fatalf("Error while parsing the integer '%s'", sNumber)
			}

			parseResult.Part1Parse[i] = append(parseResult.Part1Parse[i], number)
		}

		// Part 2 parsing:
		sNumber := strings.Join(lineSNumbers, "")
		number, err := strconv.Atoi(sNumber)
		if err != nil {
			log.Fatalf("Error while parsing the integer '%s'", sNumber)
		}

		parseResult.Part2Parse[i] = number
	}

	return parseResult
}

func (d Day) SolvePart1(timesAndRecordsI interface{}) int {
	timesAndRecords := timesAndRecordsI.(ParseResult).Part1Parse
	waysToWin := 1
	for i, raceTime := range timesAndRecords[0] {
		winFoundAt := -1
		for j := 0; j <= raceTime/2; j++ {
			distance := j * (raceTime - j)
			if distance > timesAndRecords[1][i] {
				winFoundAt = j
				break
			}
		}

		waysToWin *= raceTime - winFoundAt*2 + 1
	}

	return waysToWin
}

func (d Day) SolvePart2(timesAndRecordsI interface{}) int {
	timeAndRecord := timesAndRecordsI.(ParseResult).Part2Parse
	raceTime := timeAndRecord[0]

	winFoundAt := -1
	for i := 0; i <= raceTime/2; i++ {
		distance := i * (raceTime - i)
		if distance > timeAndRecord[1] {
			winFoundAt = i
			break
		}
	}
	return raceTime - winFoundAt*2 + 1
}
