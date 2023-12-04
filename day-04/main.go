package main

import (
	"advent-of-code-23/common"
	"advent-of-code-23/day-04/card"
	"log"
	"strconv"
	"strings"
)

func getCardsAndWinningSum(lines []string) ([]card.Card, int) {
	sum := 0

	cards := make([]card.Card, 0)
	for _, line := range lines {
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

		cards = append(cards, card.Card{
			CardID:         int(id),
			Numbers:        stringSliceToInt(sWinningNumbers),
			WinningNumbers: stringSliceToInt(sNumbers),
		})
	}

	for _, card := range cards {
		sum += card.GetWorth()
	}
	return cards, sum
}

func getFinalCardSum(cards []card.Card) int {
	notProcessedIDs := make([]int, 0)
	processedCardAmount := 0

	for _, card := range cards {
		notProcessedIDs = append(notProcessedIDs, card.CardID)
	}

	for {
		oldNotProcessedIDs := notProcessedIDs
		notProcessedIDs = make([]int, 0)
		for i := 0; i < len(oldNotProcessedIDs); i++ {
			wonIDs := cards[oldNotProcessedIDs[i]-1].GetWonCards()

			notProcessedIDs = append(notProcessedIDs, wonIDs...)
			processedCardAmount++
		}

		if len(notProcessedIDs) == 0 {
			break
		}
	}

	return processedCardAmount
}

func main() {
	lines := common.GetInputByLines("")

	cards, sum := getCardsAndWinningSum(lines)
	log.Printf("[Part 1] Sum of matching numbers: %d", sum)
	log.Printf("[Part 2] Sum of total scratchcards: %d", getFinalCardSum(cards))
}
