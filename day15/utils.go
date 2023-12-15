package day15

func getWordHash(word string) (hashVal int) {
	for _, r := range word {
		hashVal = ((hashVal + int(r)) * 17) % 256
	}
	return
}
