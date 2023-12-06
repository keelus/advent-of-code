package main

import (
	"advent-of-code-23/common"
	"flag"
	"testing"
	// import other day packages here
)

var dayNumber int
var inputFile string

func init() {
	flag.IntVar(&dayNumber, "day", 0, "The day to run the test in")
	flag.StringVar(&inputFile, "input", "input", "The file to read the puzzle input from")
}

func Benchmark(b *testing.B) {
	day := GetRunner(dayNumber)
	lines := common.GetInputByLines(inputFile, dayNumber)

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
