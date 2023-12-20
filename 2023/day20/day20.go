package day20

import (
	"strings"
)

type Day struct{}

func (d Day) GetInput(lines []string) interface{} {
	modules := make(map[string]*Module, len(lines))
	for _, line := range lines {
		parts := strings.Split(line, " -> ")

		module := parseModule(parts[0])

		connectedModules := strings.Split(parts[1], ", ")
		for _, c := range connectedModules {
			module.PulsesTo = append(module.PulsesTo, c)
		}

		modules[module.Name] = &module
	}

	return Layout{Modules: modules}
}

func parseModule(name string) Module {
	module := Module{}

	switch name[0] {
	case 'b':
		module.Type = M_BROADCASTER
		module.Name = name
	case '%':
		module.Type = M_FLIPFLOP
		module.Name = name[1:]
		module.MemFlipFlop = LOW
	case '&':
		module.Type = M_CONJUNCTION
		module.Name = name[1:]
		module.MemConjunction = make(map[string]PulseType)
	}

	module.PulsesTo = make([]string, 0)
	module.PulsesFrom = make([]string, 0)

	return module
}

func (d Day) SolvePart1(layoutI interface{}) int {
	layout := layoutI.(Layout)

	layout.initialize()

	totalSentLow, totalSentHigh := 0, 0

	for i := 0; i < 1_000; i++ {
		sentLOW, sentHIGH := layout.sendLow()
		totalSentLow += sentLOW
		totalSentHigh += sentHIGH
	}

	return totalSentLow * totalSentHigh
}

func (d Day) SolvePart2(layoutI interface{}) int {
	return -1
}
