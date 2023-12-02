package common

import (
	"bufio"
	"log"
	"os"
)

func GetInput() string {
	bContent, err := os.ReadFile("./input")
	if err != nil {
		log.Fatal("Error reading the input file")
	}

	return string(bContent)
}

func GetInputByLines() []string {
	file, err := os.Open("./input")
	if err != nil {
		log.Fatal("Error reading the input file")
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
