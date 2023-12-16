package day16

import (
	"math"
	"sync"
	"sync/atomic"
)

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

func (d Day) SolvePart1(contraptionI interface{}) (energizedAmount int) {
	contraption := contraptionI.([][]rune)

	energizedAmount = moveLight(contraption, map[Coordinate]bool{}, []Light{{Coord: Coordinate{I: 0, J: 0}, Dir: RIGHT}}, map[Visit]bool{})

	return energizedAmount
}

func (d Day) SolvePart2(contraptionI interface{}) int {
	contraption := contraptionI.([][]rune)

	var wg sync.WaitGroup

	var maxEnergizedAmount int64 = 0

	for i := 0; i < len(contraption); i++ {
		wg.Add(1)
		go func(index int) {
			defer wg.Done()
			energizedAmount := moveLight(contraption, map[Coordinate]bool{}, []Light{{Coord: Coordinate{I: index, J: 0}, Dir: RIGHT}}, map[Visit]bool{})
			atomic.StoreInt64(&maxEnergizedAmount, int64(math.Max(float64(atomic.LoadInt64(&maxEnergizedAmount)), float64(energizedAmount))))
		}(i)
	}
	wg.Wait()

	for i := 0; i < len(contraption); i++ {
		wg.Add(1)
		go func(index int) {
			defer wg.Done()
			energizedAmount := moveLight(contraption, map[Coordinate]bool{}, []Light{{Coord: Coordinate{I: index, J: len(contraption) - 1}, Dir: LEFT}}, map[Visit]bool{})
			atomic.StoreInt64(&maxEnergizedAmount, int64(math.Max(float64(atomic.LoadInt64(&maxEnergizedAmount)), float64(energizedAmount))))
		}(i)
	}

	wg.Wait()

	for i := 0; i < len(contraption[0]); i++ {
		wg.Add(1)
		go func(index int) {
			defer wg.Done()
			energizedAmount := moveLight(contraption, map[Coordinate]bool{}, []Light{{Coord: Coordinate{I: 0, J: index}, Dir: DOWN}}, map[Visit]bool{})
			atomic.StoreInt64(&maxEnergizedAmount, int64(math.Max(float64(atomic.LoadInt64(&maxEnergizedAmount)), float64(energizedAmount))))
		}(i)
	}

	wg.Wait()

	for i := 0; i < len(contraption[0]); i++ {
		wg.Add(1)
		go func(index int) {
			defer wg.Done()
			energizedAmount := moveLight(contraption, map[Coordinate]bool{}, []Light{{Coord: Coordinate{I: len(contraption) - 1, J: index}, Dir: UP}}, map[Visit]bool{})
			atomic.StoreInt64(&maxEnergizedAmount, int64(math.Max(float64(atomic.LoadInt64(&maxEnergizedAmount)), float64(energizedAmount))))
		}(i)
	}

	wg.Wait()

	return int(maxEnergizedAmount)
}
