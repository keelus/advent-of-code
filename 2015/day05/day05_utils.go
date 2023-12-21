package day05

func isNicePart1(word string) bool {
	vocals := 0
	doubleLetter := false

	for i, r := range word {
		if r == 'a' || r == 'e' || r == 'i' || r == 'o' || r == 'u' {
			vocals++
		}

		if i < len(word)-1 {
			if !doubleLetter {
				if r == rune(word[i+1]) {
					doubleLetter = true
				}
			}

			sPair := string(r) + string(word[i+1])
			if sPair == "ab" || sPair == "cd" || sPair == "pq" || sPair == "xy" {
				return false
			}
		}
	}

	return vocals >= 3 && doubleLetter
}

func isNicePart2(word string) bool {
	letterBetween := false
	sPairs := make(map[string][]int)

	for i, r := range word {
		if i < len(word)-1 {
			sPair := string(r) + string(word[i+1])

			if _, ok := sPairs[sPair]; ok {
				sPairs[sPair] = append(sPairs[sPair], i)
			} else {
				sPairs[sPair] = []int{i}
			}
		}
	}

	for i, r := range word {
		if i < len(word)-2 {
			if r == rune(word[i+2]) {
				letterBetween = true
				break
			}
		}
	}

	for _, appearances := range sPairs {
		if len(appearances) >= 2 {
			for i := 0; i < len(appearances); i++ {
				for j := 1; j < len(appearances); j++ {
					if appearances[j]-appearances[i] >= 2 { // Prevent overlap, in case it appears two times (on i and j)
						return letterBetween
					}
				}
			}
		}
	}

	return false
}

// Benchmark/Input_parsing-16                 25352             47190 ns/op
// Benchmark/Part_1-16                       704833              1810 ns/op
