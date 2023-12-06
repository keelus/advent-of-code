package main

import (
	"advent-of-code-23/common"
	"advent-of-code-23/day01"
	"advent-of-code-23/day02"
	"advent-of-code-23/day03"
	"advent-of-code-23/day04"
	"advent-of-code-23/day05"
	"advent-of-code-23/day06"
	"flag"
	"log"
	// import other day packages here
)

type Day interface {
	GetInput([]string) interface{}
	SolvePart1(interface{}) int
	SolvePart2(interface{}) int
}

func GetRunner(day int) Day {
	switch day {
	case 1:
		return day01.Day{}
	case 2:
		return day02.Day{}
	case 3:
		return day03.Day{}
	case 4:
		return day04.Day{}
	case 5:
		return day05.Day{}
	case 6:
		return day06.Day{}
	default:
		return nil
	}
}

var dayToRun Day

func main() {
	day := flag.Int("day", 1, "The day to run")
	inputFile := flag.String("input", "input", "The file to read the puzzle input from")
	flag.Parse()

	dayToRun = GetRunner(*day)
	if dayToRun != nil {
		log.Printf("Running day %d...", *day)
		lines := common.GetInputByLines(*inputFile, *day)
		input := dayToRun.GetInput(lines)
		log.Printf("Part one: %d", dayToRun.SolvePart1(input))
		log.Printf("Part two: %d", dayToRun.SolvePart2(input))
	} else {
		log.Printf("Day %d is not implemented.", *day)
	}
}
