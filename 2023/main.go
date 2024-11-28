package main

import (
	common "2023/common"
	"flag"
	"log"
)

// This file used to let you choose the year, but currently I've only done
// one advent-of-code year in Go , so is no longer required (at least for now).

// func GetYearRunner(year int) common.Year {
// 	switch year {
// 	case 2023:
// 		return Year{}
// 	default:
// 		log.Fatalf("Year %d is not defined.", year)
// 		return nil
// 	}
// }

var dayToRun common.Day

// var yearToRun common.Year

func main() {
	day := flag.Int("day", 1, "The day of the puzzle to run")
	// year := flag.Int("year", 2023, "The year of the puzzle to run")
	inputFile := flag.String("input", "input", "The file to read the puzzle input from")
	flag.Parse()

	// yearToRun = GetYearRunner(*year)
	// dayToRun = yearToRun.GetDayRunner(*day)
	dayToRun = Year{}.GetDayRunner(*day)
	if dayToRun != nil {
		log.Printf("Running day %d...", *day)
		lines := common.GetInputByLines(*inputFile /**year,*/, *day)
		input := dayToRun.GetInput(lines)
		log.Printf("Part one: %d", dayToRun.SolvePart1(input))
		log.Printf("Part two: %d", dayToRun.SolvePart2(input))
	} else {
		log.Printf("Day %d is not implemented.", *day)
	}
}
