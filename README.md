# Advent of Code 2025 🥛🍪
This repo contains my solutions for different years.
- 2023 solutions in Golang [here](./2023/).
- 2024 solutions in Rust [here](./2024/).
- 2025 solutions in C [here](./2025/).

## Days, stars and solutions 
> Benchmarks are run on an "AMD Ryzen 7 9800X3D @ 4.7/5.2GHz CPU".

| Day | Solution | Stars | Parse benchmark | Part 1 benchmark | Part 2 benchmark |
|-----|----------|-------|-------------------|-----------------|------------------|
| [Day 1:](https://adventofcode.com/2025/day/1) Secret Entrance | [Here](./2025/solutions/day01.c) | ⭐⭐ | 72µs | 24µs | 26µs |
| [Day 2:](https://adventofcode.com/2025/day/2) Gift Shop | [Here](./2025/solutions/day02.c) | ⭐⭐ | 2µs | 11ms | 72ms |
| [Day 3:](https://adventofcode.com/2025/day/3) Lobby | [Here](./2025/solutions/day03.c) | ⭐⭐ | 22µs | 15µs | 41µs |
| [Day 4:](https://adventofcode.com/2025/day/4) Printing Department | [Here](./2025/solutions/day04.c) | ⭐⭐ | 12µs | 118µs | 883µs |
| [Day 5:](https://adventofcode.com/2025/day/5) Cafeteria | [Here](./2025/solutions/day05.c) | ⭐⭐ | 95µs | 39µs | 45ns |
| [Day 6:](https://adventofcode.com/2025/day/6) Trash Compactor | [Here](./2025/solutions/day06.c) | ⭐⭐ | 28µs | 76µs | 22µs |
| [Day 7:](https://adventofcode.com/2025/day/7) Laboratories | [Here](./2025/solutions/day07.c) | ⭐⭐ | 3µs | 241µs | 469µs |
| [Day 8:](https://adventofcode.com/2025/day/8) Playground | [Here](./2025/solutions/day08.c) | ⭐⭐ | 55ms | 122µs | 762ms |

## Run it yourself
### Run a day
While being at the [2025](./2025/) directory:
```
make build
./aoc --day=<N> [--sample] [--bench=<N>]
```
> --day=\<N\>: Day to run `(1-25)`.

> --sample: Optional. If set, the input file will be the `sample.txt` located in the [inputs](./2025/inputs) folder.

> --bench=\<N\>: Optional. If set, runs the puzzle `N` times and outputs only the average execution times.

#### Example output
![Output screenshot](https://github.com/user-attachments/assets/979c7140-057b-4b7c-a6bd-64df2f41209b)
