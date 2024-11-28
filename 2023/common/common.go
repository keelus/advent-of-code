package common

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Year interface {
	GetDayRunner(int) Day
}

type Day interface {
	GetInput([]string) interface{}
	SolvePart1(interface{}) int
	SolvePart2(interface{}) int
}

func GetInputByLines(suffix string /*, year int*/, day int) []string {
	inputPath := fmt.Sprintf("./day%02d/_input/%s" /*year,*/, day, suffix)
	file, err := os.Open(inputPath)
	if err != nil {
		if os.IsNotExist(err) {
			log.Fatalf("Input file '%s' not found.", inputPath)
		} else {
			log.Fatal("Unexpected error happened while reading the input file")
		}
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	lines := []string{}
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines
}
