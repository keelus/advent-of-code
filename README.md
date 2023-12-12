# Advent of Code 2023 🎄🎁
A repo with my solutions for the daily Advent of Code puzzles, in Golang.

> The solutions are based in my AoC inputs. Yours could be different.

### Days, stars and solutions 
| Day                                                       | #1 | #1 Solution | #2 | #2 Solution    |
|-----------------------------------------------------------|----|-------------|----|----------------|
| [Day 1: Trebuchet?!](day01/main.go)                       | ⭐ | 57346      | ⭐ | 57345          |
| [Day 2: Cube Conundrum](day02/main.go)                    | ⭐ | 2377       | ⭐ | 71220          |
| [Day 3: Gear Ratios](day03/main.go)                       | ⭐ | 537732     | ⭐ | 84883664       |
| [Day 4: Scratchcards](day04/main.go)                      | ⭐ | 22674      | ⭐ | 5747443        |
| [Day 5: If You Give A Seed A Fertilizer](day05/main.go)   | ⭐ | 346433842  | ⭐ | 60294664       |
| [Day 6: Wait For It](day06/main.go)                       | ⭐ | 2374848    | ⭐ | 39132886       |
| [Day 7: Camel Cards](day07/main.go)                       | ⭐ | 252295678  | ⭐ | 250577259      |
| [Day 8: Haunted Wasteland](day08/main.go)                 | ⭐ | 18023      | ⭐ | 14449445933179 |
| [Day 9: Mirage Maintenance](day09/main.go)                | ⭐ | 1955513104 | ⭐ | 1131           |
| [Day 10: Pipe Maze](day10/main.go)                        | ⭐ | 6682       | ⭐ | 353            |
| [Day 11: Cosmic Expansion](day11/main.go)                 | ⭐ | 9556896    | ⭐ | 685038186836   |
| [Day 12: Hot Springs](day12/main.go)                      | ⭐ | 7084       |    |                 |



### Performance
| Day                                    | Input parse | #1 Runtime | #2 Runtime |
|----------------------------------------|-------------|------------|------------|
| Day 1: Trebuchet?!                     | 31.72ns     | 177.83μs   | 745.05μs   |
| Day 2: Cube Conundrum                  | 198.08μs    | 4.93μs     | 76.74ns    |
| Day 3: Gear Ratios                     | 29.14ns     | 2.62ms     | 3.49ms     |
| Day 4: Scratchcards                    | 181.83μs    | 45.84μs    | 357.59μs   |
| Day 5: If You Give A Seed A Fertilizer | 47.36μs     | 1.73μs     | 68.33s     |
| Day 6: Wait For It                     | 611.1ns     | 25.89ns    | 7.31ns     |
| Day 7: Camel Cards                     | 65.29μs     | 251.90μs   | 240.13μs   |
| Day 8: Haunted Wasteland               | 63.53μs     | 23.87μs    | 469.90μs   |
| Day 9: Mirage Maintenance              | 105.56μs    | 152.22μs   | 158.92μs   |
| Day 10: Pipe Maze                      | 3.76ms      | 19.42μs    | 458.23ms   |
| Day 11: Cosmic Expansion               | 64.26μs     | 2.56ms     | 2.57ms     |
| Day 12: Hot Springs                    | 198.19μs    | 2.25s      | -          |

## Run it yourself
### Run a day 
While being at the root directory:
```
go run . [-day=<1-25>] [-input=<file_name>]
```
> -day=<1-25> (default:`1`) Specify the day to execute the puzzle `(1-25)`

> -input=<file_name> (default:`input`) Specify the puzzle input file (located at `dayXX/.input/<file_name>`)
### Bench a day
While being at the root directory:
```
go test . -bench [-day=<1-25>] [-input=<file_name>]
```
