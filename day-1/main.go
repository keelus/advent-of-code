package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

var SPELLED_NUMBERS = [...]string{"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
var DIGIT_NUMBERS = [...]rune{'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'}

func getSpelled(index int, line string) (bool, rune) {
	if index >= len(line) {
		return false, ' '
	}

	for n, spelledWord := range SPELLED_NUMBERS {
		if index+len(spelledWord) <= len(line) {
			if spelledWord == line[index:index+len(spelledWord)] {
				return true, DIGIT_NUMBERS[n]
			}
		}
	}
	return false, ' '
}

func isNumber(char rune) bool {
	for _, number := range DIGIT_NUMBERS {
		if number == char {
			return true
		}
	}
	return false
}

func getCalibrationValue(line string, parseSpelled bool) int {
	firstNumber := ' '
	lastNumber := ' '

	for index, char := range line {
		if isNumber(char) {
			if firstNumber == ' ' {
				firstNumber = char
			}
			lastNumber = char
		} else if parseSpelled {
			isSpelled, gotChar := getSpelled(index, line)
			if isSpelled {
				if firstNumber == ' ' {
					firstNumber = gotChar
				}
				lastNumber = gotChar
			}
		}
	}

	sFinalNumber := fmt.Sprintf("%c%c", firstNumber, lastNumber)
	finalNumber, err := strconv.ParseInt(sFinalNumber, 10, 32)
	if err != nil {
		log.Fatalf("Error while parsing String to Integer for '%s'\n", sFinalNumber)
	}

	return int(finalNumber)
}

func getCalibrationSum(lines []string, parseSpelled bool) int {
	sum := 0
	for _, line := range lines {
		val := getCalibrationValue(line, parseSpelled)
		sum += val
	}

	return sum
}

func main() {
	file, err := os.Open("./input")
	if err != nil {
		log.Fatal("Error reading the input file")
	}

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	lines := []string{}
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	fmt.Printf("[Part 1] Total calibration sum: %d\n", getCalibrationSum(lines, false))
	fmt.Printf("[Part 2] Total calibration sum: %d\n", getCalibrationSum(lines, true))
}
