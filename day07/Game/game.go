package game

import (
	hand "advent-of-code-23/day07/Hand"
	"fmt"
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
	for i := 0; i < len(g.Hands); i++ {
		for j := i; j < len(g.Hands); j++ {
			comp := g.Hands[i].CompareTo(g.Hands[j], part1)
			if comp == 1 { // Sort from higher rank (1) to lower rank(n)
				g.Hands[i], g.Hands[j] = g.Hands[j], g.Hands[i]
			}
		}
	}
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
