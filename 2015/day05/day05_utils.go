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

			pair := string(r) + string(word[i+1])
			if pair == "ab" || pair == "cd" || pair == "pq" || pair == "xy" {
				return false
			}
		}
	}

	return vocals >= 3 && doubleLetter
}

func isNicePart2(word string) bool {
	letterBetween := false
	pairs := make(map[string][]int)

	for i, r := range word {
		if i < len(word)-1 {
			pair := string(r) + string(word[i+1])

			if _, ok := pairs[pair]; ok {
				pairs[pair] = append(pairs[pair], i)
			} else {
				pairs[pair] = []int{i}
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

	for _, appearances := range pairs {
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
