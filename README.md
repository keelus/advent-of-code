# Advent of Code 2023 🎄🎁
A repo with my solutions for the daily Advent of Code puzzles, in Golang.

> The solutions are based in my AoC inputs. Yours could be different.

### Days, stars and solutions 
| Day                                                       | #1 | #1 Solution | #2 | #2 Solution |
|-----------------------------------------------------------|----|-------------|----|-------------|
| [Day 1: Trebuchet?!](day-01/main.go)                      | ⭐ | 57346      | ⭐ | 57345       |
| [Day 2: Cube Conundrum](day-02/main.go)                   | ⭐ | 2377       | ⭐ | 71220       |
| [Day 3: Gear Ratios](day-03/main.go)                     	| ⭐ | 537732     | ⭐ | 84883664    |
| [Day 4: Scratchcards](day-04/main.go)                    	| ⭐ | 22674      | ⭐ | 5747443     |
| [Day 5: If You Give A Seed A Fertilizer](day-05/main.go)  | ⭐ | 346433842  | ⭐ | 60294664    |
| [Day 6: Wait For It](day-06/main.go)                      | ⭐ | 2374848    | ⭐ | 39132886    |

### Performance
| Day                                    | Input parse | #1 Runtime | #2 Runtime |
|----------------------------------------|-------------|------------|------------|
| Day 1: Trebuchet?!                     | 31.72ns     | 177.83μs   | 745.05μs   |
| Day 2: Cube Conundrum                  | 338.17μs    | 4.93μs     | 76.74ns    |
| Day 3: Gear Ratios                     | 29.14ns     | 2.62ms     | 3.49ms     |
| Day 4: Scratchcards                    | 338.38μs    | 45.84μs    | 357.59μs   |
| Day 5: If You Give A Seed A Fertilizer | 47.36μs     | 1.73μs     | 68.33s    |
| Day 6: Wait For It                     | 611.1ns     | 25.89ns    | 7.31ns     |

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
