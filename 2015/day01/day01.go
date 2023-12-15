package day01

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	return lines[0]
}

func (d Day) SolvePart1(lineI interface{}) (finalFloor int) {
	line := lineI.(string)

	for _, r := range line {
		if r == '(' {
			finalFloor++
		} else {
			finalFloor--
		}
	}

	return
}

func (d Day) SolvePart2(lineI interface{}) int {
	line := lineI.(string)

	currentFloor := 0
	for i, r := range line {
		if r == '(' {
			currentFloor++
		} else {
			currentFloor--
		}

		if currentFloor == -1 {
			return i + 1
		}
	}

	return -1
}
