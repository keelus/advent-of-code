package game

import (
	hand "advent-of-code/2023/day07/Hand"
	"fmt"
	"sort"
)

type Game struct {
	Hands []hand.Hand
}

func (g *Game) Print() {
	fmt.Printf("\n## Game status: ##\n")
	for _, h := range g.Hands {
		fmt.Printf(h.ToString() + "\n")
	}
	fmt.Printf("\n")
}

func (g *Game) SortByRank(part1 bool) {
	sort.Slice(g.Hands, func(i, j int) bool {
		return g.Hands[j].CompareTo(g.Hands[i], part1) == 1
	})
}

func (g *Game) GetWinnings() int {
	winnings := 0
	for r, h := range g.Hands {
		winnings += h.Bid * (r + 1)
	}
	return winnings
}

func (g *Game) UpdateTypes(part1 bool) {
	for i := 0; i < len(g.Hands); i++ {
		g.Hands[i].UpdateType(part1)
	}
}
