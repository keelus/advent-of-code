package day20

type ModuleType byte

const (
	M_FLIPFLOP    ModuleType = 0
	M_CONJUNCTION ModuleType = 1
	M_BROADCASTER ModuleType = 2
)

type Pulse struct {
	Type        PulseType
	Source      string
	Destination string
}

type PulseType byte

const (
	LOW  PulseType = 0
	HIGH PulseType = 1
)

type Module struct {
	Type ModuleType
	Name string

	PulsesFrom []string
	PulsesTo   []string

	MemFlipFlop    PulseType
	MemConjunction map[string]PulseType
}

func (m *Module) receivesPulse(source string, pulseType PulseType) []Pulse {
	pulses := make([]Pulse, 0, len(m.PulsesTo))
	finalType := pulseType

	switch m.Type {
	case M_CONJUNCTION:
		m.MemConjunction[source] = pulseType

		allHIGH := true
		for _, status := range m.MemConjunction {
			if status == LOW {
				allHIGH = false
				break
			}
		}

		finalType = HIGH
		if allHIGH {
			finalType = LOW
		}

	case M_FLIPFLOP:
		if pulseType == HIGH {
			return []Pulse{}
		}

		switch m.MemFlipFlop {
		case LOW:
			m.MemFlipFlop = HIGH
		case HIGH:
			m.MemFlipFlop = LOW
		}

		finalType = m.MemFlipFlop
	}

	for _, cm := range m.PulsesTo {
		pulses = append(pulses, Pulse{Source: m.Name, Destination: cm, Type: finalType})
	}

	return pulses
}

type Layout struct {
	Modules map[string]*Module
}

func (l *Layout) initialize() {
	for _, m := range l.Modules {
		for _, cm := range m.PulsesTo {
			if _, ok := l.Modules[cm]; ok {
				l.Modules[cm].PulsesFrom = append(l.Modules[cm].PulsesFrom, m.Name)

				if l.Modules[cm].Type == M_CONJUNCTION {
					l.Modules[cm].MemConjunction[m.Name] = LOW
				}
			}
		}
	}
}

func (l *Layout) sendLow() (int, int) {
	sentLOW, sentHIGH := 0, 0

	pulses := l.Modules["broadcaster"].receivesPulse("button", LOW)
	queue := append([]Pulse{}, pulses...)

	sentLOW++

	var pulse Pulse
	for len(queue) > 0 {
		pulse, queue = queue[0], queue[1:]

		if pulse.Type == LOW {
			sentLOW++
		} else {
			sentHIGH++
		}

		if _, ok := l.Modules[pulse.Destination]; ok {
			newPulses := l.Modules[pulse.Destination].receivesPulse(pulse.Source, pulse.Type)
			queue = append(queue, newPulses...)
		}
	}

	return sentLOW, sentHIGH
}
