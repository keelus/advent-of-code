package day05

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	return lines
}

func (d Day) SolvePart1(wordsI interface{}) (niceWordCount int) {
	words := wordsI.([]string)
	for _, word := range words {
		if isNicePart1(word) {
			niceWordCount++
		}
	}

	return
}

func (d Day) SolvePart2(wordsI interface{}) (niceWordCount int) {
	words := wordsI.([]string)
	for _, word := range words {
		if isNicePart2(word) {
			niceWordCount++
		}
	}

	return
}
