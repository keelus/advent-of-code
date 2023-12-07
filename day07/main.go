package day07

import (
	card "advent-of-code-23/day07/Card"
	game "advent-of-code-23/day07/Game"
	hand "advent-of-code-23/day07/Hand"
	"log"
	"strconv"
	"strings"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	game := game.Game{}
	game.Hands = make([]hand.Hand, len(lines))
	for i, line := range lines {
		newHand := &game.Hands[i]
		parts := strings.Fields(line)
		for i, c := range parts[0] {
			newHand.Cards[i] = card.Card{Letter: c}
		}

		var err error
		if newHand.Bid, err = strconv.Atoi(parts[1]); err != nil {
			log.Fatalf("Error while parsing the integer '%s'", parts[1])
		}
	}

	return game
}

func (d Day) SolvePart1(handsI interface{}) int {
	game := handsI.(game.Game)
	game.UpdateTypes(true)
	game.SortByRank(true)
	return game.GetWinnings()
}

func (d Day) SolvePart2(handsI interface{}) int {
	game := handsI.(game.Game)
	game.UpdateTypes(false)
	game.SortByRank(false)
	return game.GetWinnings()
}
