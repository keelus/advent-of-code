package day24

import (
	"log"
	"strconv"
	"strings"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	hailstones := make([]Hailstone, len(lines))
	for i, line := range lines {
		hailstone := Hailstone{P: [3]float64{0, 0, 0}, V: [3]float64{0, 0, 0}}
		parts := strings.Split(strings.ReplaceAll(line, ",", ""), " @ ")

		sPositions := strings.Fields(parts[0])
		sVelocities := strings.Fields(parts[1])

		for j, sPos := range sPositions {
			val, err := strconv.Atoi(sPos)
			if err != nil {
				log.Fatalf("Error while parsing the integer '%s'", sPos)
			}
			hailstone.P[j] = float64(val)
		}

		for j, sVel := range sVelocities {
			val, err := strconv.Atoi(sVel)
			if err != nil {
				log.Fatalf("Error while parsing the integer '%s'", sVel)
			}
			hailstone.V[j] = float64(val)
		}

		hailstones[i] = hailstone
	}
	return hailstones
}

func (d Day) SolvePart1(hailstonesI interface{}) int {
	hailstones := hailstonesI.([]Hailstone)

	collisions := 0
	for i := range hailstones {
		for j := i + 1; j < len(hailstones); j++ {
			if findCollision(hailstones[i], hailstones[j], 200000000000000, 400000000000000) {
				collisions++
			}
		}
	}

	return collisions
}

func (d Day) SolvePart2(gridI interface{}) int {
	return -1
}
