package day23

import (
	"2023/common/pair"
)

type Node struct {
	Coord     pair.Pair
	IsForest  bool
	Forces    bool
	ForcesDir pair.Pair
}

type Grid [][]Node

func (grid Grid) explorePath(currentCost int, from, wantDir, end pair.Pair) int {
	newPos := from.Add(wantDir)
	if grid[newPos.I][newPos.J].Forces {
		// We suppose that the slope makes you go in a walkable coordinate
		return grid.explorePath(currentCost+1, newPos, grid[newPos.I][newPos.J].ForcesDir, end)
	}

	newDirs := grid.getNeighborDirections(newPos, wantDir)

	if newPos.Eq(end) {
		return currentCost
	}

	maxCost := -1
	for _, targetDir := range newDirs {
		targetPos := newPos.Add(targetDir)
		targetNode := grid[targetPos.I][targetPos.J]
		if targetNode.Forces && !targetNode.ForcesDir.Eq(targetDir) { // Prevent going to a node that forces we out
			continue
		}

		cost := grid.explorePath(currentCost+1, newPos, targetDir, end)
		if cost > maxCost {
			maxCost = cost
		}
	}

	return maxCost
}

func (grid Grid) getNeighborDirections(pos, forbiddenDir pair.Pair) []pair.Pair {
	directions := []pair.Pair{pair.Up(), pair.Down(), pair.Left(), pair.Right()}
	newDirs := []pair.Pair{}
	for _, dir := range directions {
		if dir.Eq(forbiddenDir.Opp()) { // Prevent going back
			continue
		}

		newCoord := pos.Add(dir)
		if newCoord.InBounds(0, 0, len(grid), len(grid[0])) {
			if !grid[newCoord.I][newCoord.J].IsForest {
				newDirs = append(newDirs, dir)
			}
		}
	}

	return newDirs
}
