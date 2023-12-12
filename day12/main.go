package day12

import (
	"log"
	"strconv"
	"strings"
)

type Day struct{}

type Row struct {
	Record              string
	OperationalGrouping []int
}

func (d Day) GetInput(lines []string) interface{} {
	rows := make([]Row, len(lines))

	for i, line := range lines {
		parts := strings.Fields(line)

		sGrouping := strings.Split(parts[1], ",")
		grouping := make([]int, len(sGrouping))

		for j, sNumber := range sGrouping {
			number, err := strconv.Atoi(sNumber)
			if err != nil {
				log.Fatalf("There was an error parsing the integer '%s'", sNumber)
			}
			grouping[j] = number
		}

		rows[i] = Row{Record: parts[0], OperationalGrouping: grouping}
	}

	return rows
}

func (d Day) SolvePart1(rowsI interface{}) (amount int) {
	rows := rowsI.([]Row)
	for _, row := range rows {
		amount += possibleArrangmentAmount(row)
	}

	return
}

func (d Day) SolvePart2(rowsI interface{}) int {
	return -1
}
