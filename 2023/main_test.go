package main

import (
	"2023/common"
	"flag"
	"testing"
)

var dayNumber int
var yearNumber int
var inputFile string

func init() {
	flag.IntVar(&dayNumber, "day", 0, "The day of the puzzle to run")
	flag.IntVar(&yearNumber, "year", 2023, "The year of the puzzle to run")
	flag.StringVar(&inputFile, "input", "input", "The file to read the puzzle input from")
}

func Benchmark(b *testing.B) {
	year := GetYearRunner(yearNumber)
	day := year.GetDayRunner(dayNumber)
	lines := common.GetInputByLines(inputFile, yearNumber, dayNumber)

	b.ResetTimer()
	b.Run("Input parsing", func(b *testing.B) {
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			day.GetInput(lines)
		}
	})

	input := day.GetInput(lines)
	b.ResetTimer()

	b.Run("Part 1", func(b *testing.B) {
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			day.SolvePart1(input)
		}
	})
	b.Run("Part 2", func(b *testing.B) {
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			day.SolvePart2(input)
		}
	})
}
