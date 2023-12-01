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

func get_spelled(index int, line string) (bool, rune) {
	if index >= len(line) {
		return false, ' '
	}

	for n, spelled_word := range SPELLED_NUMBERS {
		if index+len(spelled_word) <= len(line) {
			if spelled_word == line[index:index+len(spelled_word)] {
				return true, DIGIT_NUMBERS[n]
			}
		}
	}
	return false, ' '
}

func is_number(char rune) bool {
	for _, number := range DIGIT_NUMBERS {
		if number == char {
			return true
		}
	}
	return false
}

func get_calibration_value(line string) int {
	first_number := ' '
	last_number := ' '

	for index, char := range line {
		if is_number(char) {
			if first_number == ' ' {
				first_number = char
			}
			last_number = char
		} else {
			is_spelled, got_char := get_spelled(index, line)
			if is_spelled {
				if first_number == ' ' {
					first_number = got_char
				}
				last_number = got_char
			}
		}
	}

	final_number_str := fmt.Sprintf("%c%c", first_number, last_number)
	final_number, err := strconv.ParseInt(final_number_str, 10, 32)
	if err != nil {
		log.Fatalf("Error while parsing String to Integer for '%s'\n", final_number_str)
	}

	return int(final_number)
}

func get_calibration_sum(lines []string) int {
	sum := 0
	for _, line := range lines {
		val := get_calibration_value(line)
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

	fmt.Printf("Total calibration sum: %d\n", get_calibration_sum(lines))
}
