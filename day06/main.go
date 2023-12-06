package day06

import (
	"log"
	"math"
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
		waysToWin *= getWinWays(raceTime, timesAndRecords[1][i])
	}

	return waysToWin
}

func (d Day) SolvePart2(timesAndRecordsI interface{}) int {
	timeAndRecord := timesAndRecordsI.(ParseResult).Part2Parse
	return getWinWays(timeAndRecord[0], timeAndRecord[1])
}

// T: Time of the race
// W: Record
func getWinWays(T int, R int) int {
	shared := float64((T*T - 4*R))
	timePressedMin := int(math.Floor(math.Abs((-float64(T) + math.Sqrt(shared)) / 2)))
	timePressedMax := int(math.Ceil(math.Abs((-float64(T) - math.Sqrt(shared)) / 2)))

	return (timePressedMax - timePressedMin) - 1
}
