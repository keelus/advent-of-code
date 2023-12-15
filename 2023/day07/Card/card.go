package card

var CardValuesPart1 map[rune]int = map[rune]int{'2': 0, '3': 1, '4': 2, '5': 3, '6': 4, '7': 5, '8': 6, '9': 7, 'T': 8, 'J': 9, 'Q': 10, 'K': 11, 'A': 12}
var CardValuesPart2 map[rune]int = map[rune]int{'J': 0, '2': 1, '3': 2, '4': 3, '5': 4, '6': 5, '7': 6, '8': 7, '9': 8, 'T': 9, 'Q': 10, 'K': 11, 'A': 12}

type Card struct {
	Letter rune
}

func (c1 Card) CompareTo(c2 Card, part1 bool) int {
	if part1 {
		if CardValuesPart1[c1.Letter] > CardValuesPart1[c2.Letter] {
			return 1
		} else if CardValuesPart1[c1.Letter] < CardValuesPart1[c2.Letter] {
			return -1
		}
	} else {
		if CardValuesPart2[c1.Letter] > CardValuesPart2[c2.Letter] {
			return 1
		} else if CardValuesPart2[c1.Letter] < CardValuesPart2[c2.Letter] {
			return -1
		}
	}

	return 0
}
