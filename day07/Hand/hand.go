package hand

import (
	card "advent-of-code-23/day07/Card"
	"fmt"
)

type HandType int

const (
	HIGH_CARD HandType = iota
	ONE_PAIR
	TWO_PAIR
	THREE_OF_A_KIND
	FULL_HOUSE
	FOUR_OF_A_KIND
	FIVE_OF_A_KIND
)

func (ht HandType) ToString() string {
	switch ht {
	case HIGH_CARD:
		return "HIGH_CARD"
	case ONE_PAIR:
		return "ONE_PAIR"
	case TWO_PAIR:
		return "TWO_PAIR"
	case THREE_OF_A_KIND:
		return "THREE_OF_A_KIND"
	case FULL_HOUSE:
		return "FULL_HOUSE"
	case FOUR_OF_A_KIND:
		return "FOUR_OF_A_KIND"
	case FIVE_OF_A_KIND:
		return "FIVE_OF_A_KIND"
	}
	return "UNKNOWN"
}

type Hand struct {
	Cards [5]card.Card
	Type  HandType
	Bid   int
}

func (h1 Hand) CompareTo(h2 Hand, part1 bool) int {
	if h1.Type > h2.Type {
		return 1
	} else if h1.Type < h2.Type {
		return -1
	}

	for i := 0; i < len(h1.Cards); i++ {
		card1 := h1.Cards[i]
		card2 := h2.Cards[i]

		result := card1.CompareTo(card2, part1)
		if result != 0 {
			return result
		}
	}
	return 0
}

func (h Hand) ToString() string {
	hString := ""
	for _, c := range h.Cards {
		hString += string(c.Letter)
	}
	hString += fmt.Sprintf("|%s|%d", h.Type.ToString(), h.Bid)

	return hString
}

func (h *Hand) UpdateType(part1 bool) {
	cardMatches := make(map[card.Card]int, 0)

	jMatches := 0

	for _, c := range h.Cards {
		if c.Letter == 'J' && !part1 {
			jMatches++
		}
		if _, ok := cardMatches[c]; ok {
			cardMatches[c]++
		} else {
			cardMatches[c] = 1
		}
	}

	switch len(cardMatches) {
	case 1:
		h.Type = FIVE_OF_A_KIND
	case 2:
		if jMatches >= 1 {
			h.Type = FIVE_OF_A_KIND
		} else {
			for _, v := range cardMatches {
				if v == 4 {
					h.Type = FOUR_OF_A_KIND
					break
				}
				if v == 3 {
					h.Type = FULL_HOUSE
					break
				}
			}
		}
	case 3:
		for _, v := range cardMatches {
			if v == 3 {
				h.Type = THREE_OF_A_KIND
				break
			}
			if v == 2 {
				h.Type = TWO_PAIR
				break
			}
		}

		if jMatches >= 1 {
			if h.Type == TWO_PAIR {
				if jMatches == 1 {
					h.Type = FULL_HOUSE
				} else {
					h.Type = FOUR_OF_A_KIND
				}
			} else {
				h.Type = FOUR_OF_A_KIND
			}
		}
	case 4:
		if jMatches >= 1 {
			h.Type = THREE_OF_A_KIND
		} else {
			h.Type = ONE_PAIR
		}
	case 5:
		if jMatches >= 1 {
			h.Type = ONE_PAIR
		} else {
			h.Type = HIGH_CARD
		}
	}
}
