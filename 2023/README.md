# Advent of Code 2023 🎄🎁
A repo with my solutions for the daily Advent of Code puzzles, in Golang.

> The solutions are based in my AoC inputs. Yours could be different.

### Days, stars and solutions 
| Day                                                           | #1 | #1 Solution | #2 | #2 Solution     |
|---------------------------------------------------------------|----|-------------|----|-----------------|
| [Day 1: Trebuchet?!](./day01/day01.go)                     | ⭐ | 57346      | ⭐ | 57345           |
| [Day 2: Cube Conundrum](./day02/day02.go)                  | ⭐ | 2377       | ⭐ | 71220           |
| [Day 3: Gear Ratios](./day03/day03.go)                     | ⭐ | 537732     | ⭐ | 84883664        |
| [Day 4: Scratchcards](./day04/day04.go)                    | ⭐ | 22674      | ⭐ | 5747443         |
| [Day 5: If You Give A Seed A Fertilizer](./day05/day05.go) | ⭐ | 346433842  | ⭐ | 60294664        |
| [Day 6: Wait For It](./day06/day06.go)                     | ⭐ | 2374848    | ⭐ | 39132886        |
| [Day 7: Camel Cards](./day07/day07.go)                     | ⭐ | 252295678  | ⭐ | 250577259       |
| [Day 8: Haunted Wasteland](./day08/day08.go)               | ⭐ | 18023      | ⭐ | 14449445933179  |
| [Day 9: Mirage Maintenance](./day09/day09.go)              | ⭐ | 1955513104 | ⭐ | 1131            |
| [Day 10: Pipe Maze](./day10/day10.go)                      | ⭐ | 6682       | ⭐ | 353             |
| [Day 11: Cosmic Expansion](./day11/day11.go)               | ⭐ | 9556896    | ⭐ | 685038186836    |
| [Day 12: Hot Springs](./day12/day12.go)                    | ⭐ | 7084       |   | -               |
| [Day 13: Point of Incidence](./day13/day13.go)             | ⭐ | 33047      | ⭐ | 28806           |
| [Day 14: Parabolic Reflector Dish](./day14/day14.go)       | ⭐ | 107142     | ⭐ | 104815          |
| [Day 15: Lens Library](./day15/main.go)                    | ⭐ | 517551     | ⭐ | 286097          |
| [Day 16: The Floor Will Be Lava](./day16/day16.go)         | ⭐ | 6855       | ⭐ | 7513            |
| [Day 17: Clumsy Crucible](./day17/day17.go)                | ⭐ | 845        | ⭐ | 993             |
| [Day 18: Lavaduct Lagoon](./day18/day18.go)                | ⭐ | 62573      | ⭐ | 54662804037719  |
| [Day 19: Aplenty](./day19/day19.go)                        | ⭐ | 319062     | ⭐ | 118638369682135 |
| [Day 20: Pulse Propagation](./day20/day20.go)              | ⭐ | 867118762  |   |                 |
| [Day 21: Step Counter](./day21/day21.go)                   | ⭐ | 3758       |   |                 |
| [Day 23: A Long Walk](./day23/day23.go)                    | ⭐ | 1930       |   |                 |
| [Day 24: Never Tell Me The Odds](./day23/day23.go)         | ⭐ | 13149      |   |                 |


### Performance
| Day                                    | Input parse | #1 Runtime | #2 Runtime |
|----------------------------------------|-------------|------------|------------|
| Day 1: Trebuchet?!                     | 31.72ns     | 177.83μs   | 745.05μs   |
| Day 2: Cube Conundrum                  | 198.08μs    | 4.93μs     | 76.74ns    |
| Day 3: Gear Ratios                     | 2.59ms      | 57.83μs    | 883.95μs   |
| Day 4: Scratchcards                    | 181.83μs    | 45.84μs    | 357.59μs   |
| Day 5: If You Give A Seed A Fertilizer | 47.36μs     | 1.73μs     | 75.17s     |
| Day 6: Wait For It                     | 611.1ns     | 25.89ns    | 7.31ns     |
| Day 7: Camel Cards                     | 65.29μs     | 251.90μs   | 240.13μs   |
| Day 8: Haunted Wasteland               | 63.53μs     | 23.87μs    | 469.90μs   |
| Day 9: Mirage Maintenance              | 105.56μs    | 152.22μs   | 158.92μs   |
| Day 10: Pipe Maze                      | 3.76ms      | 19.42μs    | 458.23ms   |
| Day 11: Cosmic Expansion               | 64.26μs     | 2.56ms     | 2.57ms     |
| Day 12: Hot Springs                    | 198.19μs    | 2.25s      | -          |
| Day 13: Point of Incidence             | 107.92μs    | 389.71μs   | 369.36μs   |
| Day 14: Parabolic Reflector Dish       | 15.70μs     | 56.54μs    | 4.24ms     |
| Day 15: Lens Library                   | 259.64μs    | 36.16μs    | 262.12μs   |
| Day 16: The Floor Will Be Lava         | 18.56μs     | 2.31ms     | 89.21ms    |
| Day 17: Clumsy Crucible                | 142.45μs    | 204.94ms   | 625.87ms   |
| Day 18: Lavaduct Lagoon                | 75.64μs     | 8.52μs     | 8.85μs     |
| Day 19: Aplenty                        | 388.29μs    | 39.96μs    | 488.24μs   |
| Day 20: Pulse Propagation              | 17.62μs     | 7.87ms     | -          |
| Day 21: Step Counter                   | 25.47μs     | 11.15ms    | -          |
| Day 23: A Long Walk                    | 289.81μs    | 7.82ms     | -          |
| Day 24: Never Tell Me The Odds         | 147.54μs    | 1.05ms     | -          |

## Run it yourself
> [!IMPORTANT]
> As I have moved this "runner" into this specific year's folder, it is no longer valid to pass
> the year as argument, so is no longer required.
### Run a day 
While being at the root directory:
```
go run . [-day=<1-25>] [-input=<file_name>] -year=2023
```
> -day=<1-25> (default:`1`) Specify the day to execute the puzzle `(1-25)`

> -input=<file_name> (default:`input`) Specify the puzzle input file (located at `./dayXX/.input/<file_name>`)
### Bench a day
While being at the root directory:
```
go test . -bench [-day=<1-25>] [-input=<file_name>] -year=2023
```

##
