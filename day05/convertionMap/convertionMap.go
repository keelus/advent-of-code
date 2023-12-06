package convertionMap

type ConvertionMap struct {
	Destination int
	Source      int
	Length      int
}

func (c *ConvertionMap) GetNumberMapped(number int) (int, bool) {
	if number >= c.Source && number <= c.Source+c.Length {
		return number - (c.Source - c.Destination), true
	}

	return -1, false
}
