package main

import (
	year2015 "advent-of-code/2015"
	year2023 "advent-of-code/2023"
	"advent-of-code/common"
	"flag"
	"log"
)

func GetYearRunner(year int) common.Year {
	switch year {
	case 2015:
		return year2015.Year{}
	case 2023:
		return year2023.Year{}
	default:
		log.Fatalf("Year %d is not defined.", year)
		return nil
	}
}

var dayToRun common.Day
var yearToRun common.Year

func main() {
	day := flag.Int("day", 1, "The day of the puzzle to run")
	year := flag.Int("year", 2023, "The year of the puzzle to run")
	inputFile := flag.String("input", "input", "The file to read the puzzle input from")
	flag.Parse()

	yearToRun = GetYearRunner(*year)
	dayToRun = yearToRun.GetDayRunner(*day)
	if dayToRun != nil {
		log.Printf("Running day %d...", *day)
		lines := common.GetInputByLines(*inputFile, *year, *day)
		input := dayToRun.GetInput(lines)
		log.Printf("Part one: %d", dayToRun.SolvePart1(input))
		log.Printf("Part two: %d", dayToRun.SolvePart2(input))
	} else {
		log.Printf("Day %d is not implemented.", *day)
	}
}
