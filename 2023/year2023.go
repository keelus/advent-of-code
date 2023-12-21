package year2023

import (
	"advent-of-code/2023/day01"
	"advent-of-code/2023/day02"
	"advent-of-code/2023/day03"
	"advent-of-code/2023/day04"
	"advent-of-code/2023/day05"
	"advent-of-code/2023/day06"
	"advent-of-code/2023/day07"
	"advent-of-code/2023/day08"
	"advent-of-code/2023/day09"
	"advent-of-code/2023/day10"
	"advent-of-code/2023/day11"
	"advent-of-code/2023/day12"
	"advent-of-code/2023/day13"
	"advent-of-code/2023/day14"
	"advent-of-code/2023/day15"
	"advent-of-code/2023/day16"
	"advent-of-code/2023/day17"
	"advent-of-code/2023/day18"
	"advent-of-code/2023/day19"
	"advent-of-code/2023/day20"
	"advent-of-code/2023/day21"
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
	case 6:
		return day06.Day{}
	case 7:
		return day07.Day{}
	case 8:
		return day08.Day{}
	case 9:
		return day09.Day{}
	case 10:
		return day10.Day{}
	case 11:
		return day11.Day{}
	case 12:
		return day12.Day{}
	case 13:
		return day13.Day{}
	case 14:
		return day14.Day{}
	case 15:
		return day15.Day{}
	case 16:
		return day16.Day{}
	case 17:
		return day17.Day{}
	case 18:
		return day18.Day{}
	case 19:
		return day19.Day{}
	case 20:
		return day20.Day{}
	case 21:
		return day21.Day{}
	default:
		log.Fatalf("Day %d is not defined.", day)
		return nil
	}
}
