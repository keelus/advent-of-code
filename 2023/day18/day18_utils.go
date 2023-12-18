package day18

type Instruction struct {
	Dir    byte
	Amount int
}

func parseTerrain(instructions []Instruction) ([][2]int, int) {
	x, y := 0, 0

	perimeter := 0
	previousDirection := byte('4')
	cornerPoints := make([][2]int, 0)

	for _, instruction := range instructions {
		if previousDirection != instruction.Dir {
			cornerPoints = append(cornerPoints, [2]int{x, y})
		}
		previousDirection = instruction.Dir

		perimeter += instruction.Amount

		switch instruction.Dir {
		case '0':
			x += instruction.Amount
		case '1':
			y += instruction.Amount
		case '2':
			x -= instruction.Amount
		case '3':
			y -= instruction.Amount
		}
	}

	cornerPoints = append(cornerPoints, [2]int{x, y})

	return cornerPoints, perimeter
}

func getTotalArea(corners [][2]int, b int) int {
	// Shoelace formula
	// 2*Area = Sum of determinants of pairs of points
	shoelace := 0
	for i := 0; i < len(corners)-2; i++ {
		shoelace += determinant(corners[i], corners[i+1])
	}

	area := shoelace / 2

	// Picks theorem
	// A = i + b/2 - 1

	i := area - b/2 + 1

	return i + b // Inside points + perimeter
}

func determinant(a, b [2]int) int {
	return a[0]*b[1] - a[1]*b[0]
}
