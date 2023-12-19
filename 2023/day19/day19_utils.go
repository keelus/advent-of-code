package day19

type Category byte

type Operation byte

type Rule struct {
	Cat         Category
	Op          Operation
	Val         int
	Destination string
}

type Workflow struct {
	Name      string
	Rules     []Rule
	Exception string
}

type Part struct {
	Values map[Category]int
}

func (wf Workflow) parsePart(wfs map[string]Workflow, part Part) bool {
	for _, rule := range wf.Rules {
		if (rule.Op == '<' && part.Values[rule.Cat] < rule.Val) || (rule.Op == '>' && part.Values[rule.Cat] > rule.Val) {
			switch rule.Destination {
			case "A", "R":
				return rule.Destination == "A"
			default:
				return wfs[rule.Destination].parsePart(wfs, part)
			}
		}
	}

	switch wf.Exception {
	case "A", "R":
		return wf.Exception == "A"
	default:
		return wfs[wf.Exception].parsePart(wfs, part)
	}
}
