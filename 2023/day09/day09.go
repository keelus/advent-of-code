package day09

import (
	"log"
	"strconv"
	"strings"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	dataset := make([][][]int, len(lines))

	for i, line := range lines {
		sNumbers := strings.Fields(line)
		dataset[i] = append(dataset[i], make([]int, len(sNumbers)))

		for j, sNum := range sNumbers {
			if num, err := strconv.Atoi(sNum); err != nil {
				log.Fatalf("There was an error parsing the integer '%s'", sNum)
			} else {
				dataset[i][0][j] = num
			}
		}
	}

	return dataset
}

func (d Day) SolvePart1(datasetInterface interface{}) int {
	sum := 0
	dataset := datasetInterface.([][][]int)

	for _, sequences := range dataset {
		previousSequence := sequences[len(sequences)-1]
		for !isLastSequence(previousSequence) {
			newSequence := make([]int, len(previousSequence)-1)

			for j := 0; j < len(previousSequence)-1; j++ {
				newSequence[j] = previousSequence[j+1] - previousSequence[j]

				if j == len(previousSequence)-2 {
					sum += previousSequence[j+1]
				}
			}

			sequences = append(sequences, newSequence)
			previousSequence = newSequence
		}
	}

	return sum
}

func (d Day) SolvePart2(datasetInterface interface{}) int {
	sum := 0
	dataset := datasetInterface.([][][]int)

	for _, sequences := range dataset {
		previousSequence := sequences[len(sequences)-1]
		for i := 0; !isLastSequence(previousSequence); i++ {
			newSequence := make([]int, len(previousSequence)-1)

			for j := 0; j < len(previousSequence)-1; j++ {
				newSequence[j] = previousSequence[j+1] - previousSequence[j]

				if j == 0 {
					sum += previousSequence[j] * (1 - 2*(i%2))
				}
			}

			sequences = append(sequences, newSequence)
			previousSequence = newSequence
		}
	}

	return sum
}

func isLastSequence(sequence []int) bool {
	for _, num := range sequence {
		if num != 0 {
			return false
		}
	}

	return true
}
