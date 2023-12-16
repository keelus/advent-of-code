package year2015

import (
	"advent-of-code/2015/day01"
	"advent-of-code/2015/day02"
	"advent-of-code/2015/day03"
	"advent-of-code/2015/day04"
	"advent-of-code/2015/day05"
	"advent-of-code/common"
	"log"
)

type Year struct{}

func (year Year) GetDayRunner(day int) common.Day {
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
	default:
		log.Fatalf("Day %d is not defined.", day)
		return nil
	}
}
