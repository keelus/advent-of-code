# Advent of Code 2015 🎄🎁
A repo with my solutions for the daily Advent of Code puzzles, in Golang.

> The solutions are based in my AoC inputs. Yours could be different.

### Days, stars and solutions 
| Day                                                            | #1 | #1 Solution | #2 | #2 Solution    |
|----------------------------------------------------------------|----|-------------|----|----------------|
| [Day 1: Not Quite Lisp](day01/main.go)                         | ⭐ | 232        | ⭐ | 1783           |
| [Day 2: I Was Told There Would Be No Math](day02/main.go)      | ⭐ | 1606483    | ⭐ | 3842356        |
| [Day 3: Perfectly Spherical Houses in a Vacuum](day03/main.go) | ⭐ | 2565       | ⭐ | 2639           |
| [Day 4: The Ideal Stocking Stuffer](day04/main.go)             | ⭐ | 254575     | ⭐ | 1038736        |


### Performance
| Day                                           | Input parse | #1 Runtime | #2 Runtime  |
|-----------------------------------------------|-------------|------------|-------------|
| Day 1: Not Quite Lisp                         | 27.22ns     | 3892ns     | 1174ns      |
| Day 2: I Was Told There Would Be No Math      | 80973ns     | 7771ns     | 12901ns     |
| Day 3: Perfectly Spherical Houses in a Vacuum | 26.69ns     | 1456518ns  | 1550624ns   |
| Day 4: The Ideal Stocking Stuffer             | 26.87ns     | 71442194ns | 289916375ns |

## Run it yourself
### Run a day 
While being at the root directory:
```
go run . [-day=<1-25>] [-input=<file_name>] -year=2015
```
> -day=<1-25> (default:`1`) Specify the day to execute the puzzle `(1-25)`

> -input=<file_name> (default:`input`) Specify the puzzle input file (located at `2015/dayXX/.input/<file_name>`)
### Bench a day
While being at the root directory:
```
go test . -bench [-day=<1-25>] [-input=<file_name>] -year=2015
```
