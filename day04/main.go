package day04

import (
	"advent-of-code-23/day04/card"
	"log"
	"strconv"
	"strings"
)

type Day struct{}

func getLineCard(line string) card.Card {
	parts := strings.Split(line, ": ")

	sId := strings.ReplaceAll(parts[0], "Card", "")
	id, err := strconv.ParseInt(strings.TrimSpace(sId), 10, 64)
	if err != nil {
		log.Fatalf("There was an error parsing the integer '%s'", sId)
	}

	numberParts := strings.Split(parts[1], " | ")
	sWinningNumbers := strings.Split(strings.TrimSpace(numberParts[0]), " ")
	sNumbers := strings.Split(strings.TrimSpace(numberParts[1]), " ")

	stringSliceToInt := func(sSlice []string) []int {
		integerSlice := make([]int, 0)
		for _, sElement := range sSlice {
			if strings.TrimSpace(sElement) == "" {
				continue
			}
			element, err := strconv.ParseInt(sElement, 10, 64)
			if err != nil {
				log.Fatalf("There was an error parsing a number of the slice to integer '%s'", sElement)
			}
			integerSlice = append(integerSlice, int(element))
		}
		return integerSlice
	}

	return card.Card{
		CardID:         int(id),
		Numbers:        stringSliceToInt(sWinningNumbers),
		WinningNumbers: stringSliceToInt(sNumbers),
	}
}

func (d Day) GetInput(lines []string) interface{} {
	cards := make([]card.Card, 0)
	for _, line := range lines {
		card := getLineCard(line)
		cards = append(cards, card)

	}

	return cards
}

func (d Day) SolvePart1(cardsI interface{}) int {
	cards := cardsI.([]card.Card)
	sum := 0
	for _, card := range cards {
		sum += card.GetWorth()
	}
	return sum
}

func (d Day) SolvePart2(cardsI interface{}) int {
	cards := cardsI.([]card.Card)
	wonCardAmounts := make([]int, len(cards))

	for i := len(cards) - 1; i >= 0; i-- {
		wonCards := cards[i].GetWonCards()

		wonCardAmounts[i] = len(wonCards) // Save how many cars this card wins

		for _, j := range wonCards {
			wonCardAmounts[i] += wonCardAmounts[j-1] // Add how many cards each won card wins
		}
	}

	sum := 0
	for _, val := range wonCardAmounts {
		sum += val + 1 // The card's won card amount + the card itself
	}
	return sum
}
