package card

import "math"

type Card struct {
	CardID         int
	WinningNumbers []int
	Numbers        []int
}

func (card *Card) getMatches() []int {
	matches := make([]int, 0)
	for _, number := range card.Numbers {
		for _, winningNumber := range card.WinningNumbers {
			if number == winningNumber {
				matches = append(matches, number)
				break
			}
		}
	}

	return matches
}

func (card *Card) GetWorth() int {
	matches := card.getMatches()
	return int(math.Pow(2, float64(len(matches)-1)))
}

func (card *Card) GetWonCards() []int {
	wonCards := make([]int, 0)
	for i := card.CardID + 1; i < card.CardID+len(card.getMatches())+1; i++ {
		wonCards = append(wonCards, i)
	}

	return wonCards
}
