package day04

import (
	"advent-of-code/2023/day04/card"
	"log"
	"strconv"
	"strings"
)

type Day struct{}

func getLineCard(line string) card.Card {
	parts := strings.Split(line, ": ")

	sId := strings.ReplaceAll(parts[0], "Card", "")
	id, err := strconv.Atoi(strings.TrimSpace(sId))
	if err != nil {
		log.Fatalf("There was an error parsing the integer '%s'", sId)
	}

	numberParts := strings.Split(parts[1], " | ")
	sWinningNumbers := strings.Fields(numberParts[0])
	sNumbers := strings.Fields(numberParts[1])

	stringSliceToInt := func(sSlice []string) []int {
		integerSlice := make([]int, len(sSlice))
		for i, sElement := range sSlice {
			element, err := strconv.Atoi(sElement)
			if err != nil {
				log.Fatalf("There was an error parsing a number of the slice to integer '%s'", sElement)
			}
			integerSlice[i] = element
		}
		return integerSlice
	}

	return card.Card{
		CardID:         id,
		Numbers:        stringSliceToInt(sWinningNumbers),
		WinningNumbers: stringSliceToInt(sNumbers),
	}
}

func (d Day) GetInput(lines []string) interface{} {
	cards := make([]card.Card, len(lines))
	for i, line := range lines {
		card := getLineCard(line)
		cards[i] = card
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

	sum := 0
	for i := len(cards) - 1; i >= 0; i-- {
		wonCards := cards[i].GetWonCards()

		wonCardAmounts[i] = len(wonCards) // Save how many cards this card wins

		for _, j := range wonCards {
			wonCardAmounts[i] += wonCardAmounts[j-1] // Add how many cards each won card wins
		}

		sum += wonCardAmounts[i]
	}

	return sum + len(cards) // Also sum the original cards
}
