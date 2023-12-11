package day11

type Day struct{}

type Coordinate struct {
	I, J int
}

type ParsedInput struct {
	ExpansionCols, ExpansionRows []int
	Galaxies                     []Coordinate
}

func (d Day) GetInput(lines []string) interface{} {
	universe := lines
	expansionRows, expansionCols := getUniverseExpansions(universe)
	galaxies := getGalaxies(universe)

	return ParsedInput{
		ExpansionCols: expansionCols,
		ExpansionRows: expansionRows,
		Galaxies:      galaxies,
	}
}

func (d Day) SolvePart1(parsedInputI interface{}) (steps int) {
	parsedInput := parsedInputI.(ParsedInput)

	for i := 0; i < len(parsedInput.Galaxies); i++ {
		for j := i + 1; j < len(parsedInput.Galaxies); j++ {
			steps += calcDistance(parsedInput.Galaxies[i], parsedInput.Galaxies[j], parsedInput.ExpansionCols, parsedInput.ExpansionRows, 2)
		}
	}

	return
}

func (d Day) SolvePart2(parsedInputI interface{}) (steps int) {
	parsedInput := parsedInputI.(ParsedInput)

	for i := 0; i < len(parsedInput.Galaxies); i++ {
		for j := i + 1; j < len(parsedInput.Galaxies); j++ {
			steps += calcDistance(parsedInput.Galaxies[i], parsedInput.Galaxies[j], parsedInput.ExpansionCols, parsedInput.ExpansionRows, 1_000_000)
		}
	}

	return
}
