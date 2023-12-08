package day08

import (
	"math"
)

type Day struct{}

type InputResult struct {
	Instructions string
	Nodes        []Node
}
type Node struct {
	Name  string
	Left  *Node
	Right *Node
}

func (d Day) GetInput(lines []string) interface{} {
	nodes := make([]Node, len(lines)-2)
	nodesHelper := make(map[string]*Node, len(lines)-2)

	for i := 2; i < len(lines); i++ {
		origin := lines[i][:3]
		nodesHelper[origin] = &nodes[i-2]
		nodes[i-2].Name = origin
	}

	for i := 2; i < len(lines); i++ {
		left := lines[i][7:10]
		right := lines[i][12:15]
		currentNode := &nodes[i-2]

		if node, ok := nodesHelper[left]; ok {
			currentNode.Left = node
		}
		if node, ok := nodesHelper[right]; ok {
			currentNode.Right = node
		}
	}

	return InputResult{Instructions: lines[0], Nodes: nodes}
}

func (d Day) SolvePart1(inputI interface{}) int {
	input := inputI.(InputResult)

	var currentNode *Node = nil // Starting node
	for _, node := range input.Nodes {
		if node.Name == "AAA" {
			currentNode = &node
			break
		}
	}

	steps := 0
	for {
		for _, dir := range input.Instructions {
			if dir == 'L' {
				currentNode = currentNode.Left
			} else {
				currentNode = currentNode.Right
			}
			steps++

			if currentNode.Name == "ZZZ" {
				return steps
			}
		}
	}
}

func (d Day) SolvePart2(inputI interface{}) int {
	input := inputI.(InputResult)

	var currentNodes []Node = make([]Node, 0) // Starting nodes
	for _, node := range input.Nodes {
		if node.Name[2] == 'A' {
			currentNodes = append(currentNodes, node)
		}
	}

	var steps []int = make([]int, len(currentNodes))

	for i := 0; i < len(currentNodes); i++ {
		ended := false
		for {
			for _, dir := range input.Instructions {
				if dir == 'L' {
					currentNodes[i] = *currentNodes[i].Left
				} else {
					currentNodes[i] = *currentNodes[i].Right
				}
				steps[i]++

				if currentNodes[i].Name[2] == 'Z' {
					ended = true
					break
				}
			}
			if ended {
				break
			}
		}
	}

	return calculateLCM(steps)
}

func calculateLCM(nums []int) int {
	primeFactorCounts := make(map[int]map[int]int, 0)

	for _, num := range nums {
		currentNumber := num
		primeFactorCounts[num] = make(map[int]int, 0)
		for i := 2; i <= num; i++ {
			count := 0
			for {
				result := currentNumber % i
				if result == 0 {
					count++
					currentNumber /= i
				} else {
					break
				}
			}
			primeFactorCounts[num][i] = count
			if currentNumber == 1 {
				break
			}
		}
	}

	maxOccurrences := make(map[int]int, 100)
	for _, counts := range primeFactorCounts {
		for prime, occurrence := range counts {
			if occurrence == 0 {
				continue
			}
			if occurrence > maxOccurrences[prime] {
				maxOccurrences[prime] = occurrence
			}
		}
	}

	result := 1
	for prime, occurrence := range maxOccurrences {
		result *= int(math.Pow(float64(prime), float64(occurrence)))
	}

	return result
}
