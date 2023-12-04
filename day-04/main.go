package main

import (
	"advent-of-code-23/common"
	"advent-of-code-23/day-04/card"
	"log"
	"strconv"
	"strings"
)

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

func getCardsAndPoints(lines []string) ([]card.Card, int) {
	sum := 0

	cards := make([]card.Card, 0)
	for _, line := range lines {
		card := getLineCard(line)
		cards = append(cards, card)

	}

	for _, card := range cards {
		sum += card.GetWorth()
	}
	return cards, sum
}

func getFinalCardSum(cards []card.Card) int {
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

func main() {
	lines := common.GetInputByLines("")

	cards, points := getCardsAndPoints(lines)
	log.Printf("[Part 1] Points worth: %d", points)
	log.Printf("[Part 2] Total scratchcards: %d", getFinalCardSum(cards))
}
