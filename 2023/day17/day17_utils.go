package day17

import (
	"log"

	"golang.org/x/exp/maps"
	"golang.org/x/exp/slices"
)

type Direction uint8

const (
	UP    Direction = 0
	DOWN  Direction = 1
	LEFT  Direction = 2
	RIGHT Direction = 3
	NODIR Direction = 4
)

type Coordinate struct {
	I int
	J int
}

type Node struct {
	Coord Coordinate
	Cost  int
}

type Visit struct {
	Coord                 Coordinate
	HeatLoss, Accumulator int
	Dir                   Direction
}

type PriorityQueue map[int][]Visit

type Visited map[Visit]bool

func (pq PriorityQueue) AddWithPriority(visit Visit) {
	cost := visit.HeatLoss

	if _, ok := pq[cost]; ok {
		pq[cost] = append(pq[cost], visit)
	} else {
		pq[cost] = []Visit{visit}
	}
}

func (pq PriorityQueue) Pop() Visit {
	minimumCost := slices.Min(maps.Keys(pq))

	minimum := pq[minimumCost][0]
	pq[minimumCost] = pq[minimumCost][1:]

	if len(pq[minimumCost]) == 0 {
		delete(pq, minimumCost)
	}

	return minimum
}

func NewVisit(i, j, d int, dir Direction, acc int) Visit {
	return Visit{Coord: Coordinate{I: i, J: j}, HeatLoss: d, Dir: dir, Accumulator: acc}
}

func (s Visited) Contains(visit Visit) bool {
	if _, ok := s[visit]; ok {
		return true
	}
	return false
}

func (s Visited) Add(visit Visit) {
	s[visit] = true
}

var (
	ROWS = -1
	COLS = -1
)

func shortestPathCost(island [][]Node, source Coordinate, dest Coordinate, minAccumulator, maxAccumulator int) int {
	ROWS = len(island)
	COLS = len(island[0])

	visited := make(Visited)
	pq := make(PriorityQueue)

	di, dj := dest.I, dest.J

	pq.AddWithPriority(NewVisit(0, 0, 0, NODIR, 0))

	for len(pq) > 0 {
		u := pq.Pop()

		if u.Coord.I == di && u.Coord.J == dj && u.Accumulator >= minAccumulator {
			return u.HeatLoss
		}

		if visited.Contains(u) {
			continue
		}

		visited.Add(u)

		if u.Accumulator < maxAccumulator && u.Dir != NODIR {
			newPos := nextPos(u.Coord, u.Dir)
			ni, nj := newPos.I, newPos.J
			if ni >= 0 && ni < ROWS && nj >= 0 && nj < COLS {
				pq.AddWithPriority(NewVisit(ni, nj, u.HeatLoss+island[ni][nj].Cost, u.Dir, u.Accumulator+1))
			}
		}

		if u.Accumulator >= minAccumulator || u.Dir == NODIR {
			for dir, v := range GetNeighbors(u.Coord, island, u.Dir) {
				if v != nil {
					pq.AddWithPriority(NewVisit(v.Coord.I, v.Coord.J, u.HeatLoss+v.Cost, dir, 1))
				}
			}
		}
	}

	log.Print("We couldn't get to an end")
	return -1
}

func nextPos(coord Coordinate, dir Direction) Coordinate {
	switch dir {
	case UP:
		return Coordinate{I: coord.I - 1, J: coord.J}
	case DOWN:
		return Coordinate{I: coord.I + 1, J: coord.J}
	case LEFT:
		return Coordinate{I: coord.I, J: coord.J - 1}
	case RIGHT:
		return Coordinate{I: coord.I, J: coord.J + 1}
	}

	log.Fatalf("Error NODIR got in nextPos")
	return Coordinate{-1, -1}
}

func GetNeighbors(coord Coordinate, island [][]Node, forbiddenDir Direction) map[Direction]*Node {
	i := coord.I
	j := coord.J

	neighbors := map[Direction]*Node{}

	if forbiddenDir != UP && forbiddenDir != DOWN {
		if i-1 >= 0 {
			neighbors[UP] = &island[i-1][j]
		}
		if i+1 < ROWS {
			neighbors[DOWN] = &island[i+1][j]
		}
	}

	if forbiddenDir != LEFT && forbiddenDir != RIGHT {
		if j-1 >= 0 {
			neighbors[LEFT] = &island[i][j-1]
		}

		if j+1 < COLS {
			neighbors[RIGHT] = &island[i][j+1]
		}
	}

	return neighbors
}
