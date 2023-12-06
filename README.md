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

### Performance
| Day                                    | Input parse | #1 Runtime | #2 Runtime |
|----------------------------------------|-------------|------------|------------|
| Day 1: Trebuchet?!                     | 31.72ns     | 177.83μs   | 745.05μs   |
| Day 2: Cube Conundrum                  | 338.17μs    | 4.93μs     | 76.74ns    |
| Day 3: Gear Ratios                     | 29.14ns     | 2.62ms     | 3.49ms     |
| Day 4: Scratchcards                    | 338.38μs    | 45.84μs    | 357.59μs   |
| Day 5: If You Give A Seed A Fertilizer | 47.36μs     | 1.73μs     | 210.81s    |

## Run it yourself
Both the run and test executions will use the input file that is located on each day's folder.
### Run a day 
While being at the root directory:
```bash
go run . -day=X #Replace X with the day you want to execute [1-25]
```

### Bench a day
While being at the root directory:
```bash
go test . -bench -day=X
```