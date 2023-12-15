package day02

import (
	"log"
	"strconv"
	"strings"
)

type Day struct{}

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

func (game Game) isPossible() bool {
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

	sGameId := strings.TrimPrefix(gameParts[0], "Game")
	gameId, err := strconv.Atoi(sGameId)
	if err != nil {
		log.Fatalf("Error while parsing the game id to integer '%s'", sGameId)
	}

	minCubes := map[string]int{"red": 0, "green": 0, "blue": 0}

	sCubeSets := strings.Split(gameParts[1], ";")

	game := Game{ID: gameId, Sets: make([]CubeSet, len(sCubeSets))}

	for i, sets := range sCubeSets {
		sCubes := strings.Split(sets, ",")
		cubeSet := CubeSet{Cubes: make([]Cube, len(sCubes))}
		for j, sCube := range sCubes {
			var color string
			for _, aColor := range COLORS {
				if strings.Contains(sCube, aColor) {
					color = aColor
					break
				}
			}

			sAmount := strings.TrimRight(sCube, color)
			amount, err := strconv.Atoi(sAmount)
			if err != nil {
				log.Fatalf("Erro while parsing the cube amount integer '%s'", sAmount)
			}

			if amount > minCubes[color] {
				minCubes[color] = amount
			}

			cube := Cube{Color: color, Amount: amount}
			cubeSet.Cubes[j] = cube
		}
		game.Sets[i] = cubeSet
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

func (d Day) GetInput(lines []string) interface{} {
	games := make([]Game, len(lines))
	for i, line := range lines {
		game := parseGame(line)
		games[i] = game
	}
	return games
}

func (d Day) SolvePart1(gamesI interface{}) int {
	games := gamesI.([]Game)
	sum := 0
	for _, game := range games {
		if game.isPossible() {
			sum += game.ID
		}
	}

	return sum
}

func (d Day) SolvePart2(gamesI interface{}) int {
	games := gamesI.([]Game)
	sumofPowers := 0
	for _, game := range games {
		sumofPowers += game.Power
	}

	return sumofPowers
}
