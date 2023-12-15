package day04

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	return lines[0]
}

func (d Day) SolvePart1(secretKeyI interface{}) int {
	secretKey := secretKeyI.(string)
	return findMd5Hash(secretKey, "00000")
}

func (d Day) SolvePart2(secretKeyI interface{}) int {
	secretKey := secretKeyI.(string)
	return findMd5Hash(secretKey, "000000")
}
