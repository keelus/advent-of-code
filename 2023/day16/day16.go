package day16

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	contraption := make([][]rune, len(lines))
	for i, line := range lines {
		contraption[i] = make([]rune, len(line))
		for j, elem := range line {
			contraption[i][j] = elem
		}
	}

	return contraption
}

func (d Day) SolvePart1(contraptionI interface{}) int {
	contraption := contraptionI.([][]rune)

	energized := make([][]rune, len(contraption))
	for i, line := range contraption {
		energized[i] = make([]rune, len(line))
	}

	moveLight(contraption, energized, []Light{{I: 0, J: 0, Dir: RIGHT}}, 0, 0)

	return getEnergizedAmount(energized)
}

func (d Day) SolvePart2(contraptionI interface{}) int {
	return -1
}
