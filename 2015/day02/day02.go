package day02

import (
	"log"
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	presents := make([][3]int, len(lines))

	for i, line := range lines {
		parts := strings.Split(line, "x")

		for j, sNum := range parts {
			num, err := strconv.Atoi(sNum)
			if err != nil {
				log.Printf("Error while parsing the integer '%s'", sNum)
			}
			presents[i][j] = num
		}
	}

	return presents
}

func (d Day) SolvePart1(presentsI interface{}) (paper int) {
	presents := presentsI.([][3]int)

	for _, present := range presents {
		sides := make([]int, 3)
		sides[0] = present[0] * present[1]
		sides[1] = present[0] * present[2]
		sides[2] = present[1] * present[2]
		paper += slices.Min(sides) + 2*sides[0] + 2*sides[1] + 2*sides[2]
	}

	return
}

func (d Day) SolvePart2(presentsI interface{}) (ribbon int) {
	presents := presentsI.([][3]int)

	for _, present := range presents {
		sizes := []int{present[0], present[1], present[2]}
		slices.Sort(sizes)
		ribbon += 2*sizes[0] + 2*sizes[1] + sizes[0]*sizes[1]*sizes[2]
	}

	return
}
