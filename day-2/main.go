package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

var MAX_CUBES = map[string]int{"red": 12, "green": 13, "blue": 14}
var COLORS = []string{"red", "green", "blue"}

type Cube struct {
	Color  string
	Amount int
}

type CubeSet struct {
	Cubes []Cube
}

type Game struct {
	ID    int
	Sets  []CubeSet
	Power int
}

func (game *Game) isPossible() bool {
	for _, set := range game.Sets {
		for _, cube := range set.Cubes {
			if cube.Amount > MAX_CUBES[cube.Color] {
				return false
			}
		}
	}

	return true
}

func parseGame(gameStr string) Game {
	gameParts := strings.Split(strings.ReplaceAll(gameStr, " ", ""), ":")

	sGameId := strings.Replace(gameParts[0], "Game", "", 1)
	gameId, err := strconv.ParseInt(sGameId, 10, 64)
	if err != nil {
		log.Fatalf("Erro while parsing the game id to integer '%s'", sGameId)
	}

	game := Game{ID: int(gameId), Sets: make([]CubeSet, 0)}
	minCubes := map[string]int{"red": 0, "green": 0, "blue": 0}

	for _, sets := range strings.Split(gameParts[1], ";") {
		cubeSet := CubeSet{Cubes: make([]Cube, 0)}
		for _, sCube := range strings.Split(sets, ",") {
			color := ""
			for _, aColor := range COLORS {
				if strings.Contains(sCube, aColor) {
					color = aColor
				}
			}

			sAmount := strings.Replace(sCube, color, "", 1)
			amount, err := strconv.ParseInt(sAmount, 10, 64)
			if err != nil {
				log.Fatalf("Erro while parsing the cube amount integer '%s'", sAmount)
			}

			if int(amount) > minCubes[color] {
				minCubes[color] = int(amount)
			}

			cube := Cube{Color: color, Amount: int(amount)}
			cubeSet.Cubes = append(cubeSet.Cubes, cube)
		}
		game.Sets = append(game.Sets, cubeSet)
	}

	game.Power = 1
	for _, minAmount := range minCubes {
		game.Power *= minAmount
	}
	return game
}

func parseGames(lines []string) (int, int) {
	sum := 0
	sumofPowers := 0
	for _, gameLine := range lines {
		game := parseGame(gameLine)
		if game.isPossible() {
			sum += game.ID
		}
		sumofPowers += game.Power
	}

	return sum, sumofPowers
}

func main() {
	file, err := os.Open("./input")
	if err != nil {
		log.Fatal("Error reading the input file")
	}

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	lines := []string{}
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	sum, sumOfPowers := parseGames(lines)
	log.Printf("[Part 1] Possible game sum: %d", sum)
	log.Printf("[Part 2] Games power sum: %d", sumOfPowers)
}
