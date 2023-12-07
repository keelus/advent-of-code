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
	for _, line := range lines {
		newHand := hand.Hand{}
		parts := strings.Fields(line)
		for i, c := range parts[0] {
			newHand.Cards[i] = card.Card{Letter: c}
		}

		bid, err := strconv.Atoi(parts[1])
		if err != nil {
			log.Fatalf("Error while parsing the integer '%s'", parts[1])
		}
		newHand.Bid = bid

		game.Hands = append(game.Hands, newHand)
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
