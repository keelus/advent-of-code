package common

import (
	"bufio"
	"fmt"
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

func GetInputByLines(suffix string) []string {
	file, err := os.Open(fmt.Sprintf("./input%s", suffix))
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
