# Advent of Code 2023 🎄🎁
A repo with my solutions for the daily Advent of Code puzzles, in Golang.

> The solutions are based in my AoC inputs. Yours could be different.

### Days, stars and solutions 
| Day                                                       | #1 | #1 Solution | #2 | #2 Solution    |
|-----------------------------------------------------------|----|-------------|----|----------------|
| [Day 1: Trebuchet?!](2023/day01/main.go)                       | ⭐ | 57346      | ⭐ | 57345          |
| [Day 2: Cube Conundrum](2023/day02/main.go)                    | ⭐ | 2377       | ⭐ | 71220          |
| [Day 3: Gear Ratios](2023/day03/main.go)                       | ⭐ | 537732     | ⭐ | 84883664       |
| [Day 4: Scratchcards](2023/day04/main.go)                      | ⭐ | 22674      | ⭐ | 5747443        |
| [Day 5: If You Give A Seed A Fertilizer](2023/day05/main.go)   | ⭐ | 346433842  | ⭐ | 60294664       |
| [Day 6: Wait For It](2023/day06/main.go)                       | ⭐ | 2374848    | ⭐ | 39132886       |
| [Day 7: Camel Cards](2023/day07/main.go)                       | ⭐ | 252295678  | ⭐ | 250577259      |
| [Day 8: Haunted Wasteland](2023/day08/main.go)                 | ⭐ | 18023      | ⭐ | 14449445933179 |
| [Day 9: Mirage Maintenance](2023/day09/main.go)                | ⭐ | 1955513104 | ⭐ | 1131           |
| [Day 10: Pipe Maze](2023/day10/main.go)                        | ⭐ | 6682       | ⭐ | 353            |
| [Day 11: Cosmic Expansion](2023/day11/main.go)                 | ⭐ | 9556896    | ⭐ | 685038186836   |
| [Day 12: Hot Springs](2023/day12/main.go)                      | ⭐ | 7084       |   |                |
| [Day 13: Point of Incidence](2023/day13/main.go)               | ⭐ | 33047      | ⭐ | 28806          |
| [Day 14: Parabolic Reflector Dish](2023/day14/main.go)         | ⭐ | 107142     | ⭐ | 104815         |
| [Day 15: Lens Library](2023/day15/main.go)                     | ⭐ | 517551     | ⭐ | 286097         |
| [Day 16: The Floor Will Be Lava](2023/day16/main.go)           | ⭐ | 6855       |   |                |


### Performance
| Day                                    | Input parse | #1 Runtime | #2 Runtime |
|----------------------------------------|-------------|------------|------------|
| Day 1: Trebuchet?!                     | 31.72ns     | 177.83μs   | 745.05μs   |
| Day 2: Cube Conundrum                  | 198.08μs    | 4.93μs     | 76.74ns    |
| Day 3: Gear Ratios                     | 2.59ms      | 57.83μs    | 883.95μs   |
| Day 4: Scratchcards                    | 181.83μs    | 45.84μs    | 357.59μs   |
| Day 5: If You Give A Seed A Fertilizer | 47.36μs     | 1.73μs     | 68.33s     |
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
| Day 16: The Floor Will Be Lava         | 18.56μs     | 15.38s     | -          |

## Run it yourself
### Run a day 
While being at the root directory:
```
go run . [-day=<1-25>] [-input=<file_name>] -year=2023
```
> -day=<1-25> (default:`1`) Specify the day to execute the puzzle `(1-25)`

> -input=<file_name> (default:`input`) Specify the puzzle input file (located at `2023/dayXX/.input/<file_name>`)
### Bench a day
While being at the root directory:
```
go test . -bench [-day=<1-25>] [-input=<file_name>] -year=2023
```

##