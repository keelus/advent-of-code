# Advent of Code 2024 🎄🎁
This repo contains my solutions for different years.
- 2023 solutions in Golang [here](./2023/).
- 2024 solutions in Rust [here](./2024/).

## Days, stars and solutions 
> Benchmarks are run on an "Intel i7-10700K @ 5.100Ghz CPU".

| Day | Solution | Stars | Parse benchmark | Part 1 benchmark | Part 2 benchmark |
|-----|----------|-------|-------------------|-----------------|------------------|
| [Day 1:](https://adventofcode.com/2024/day/1) Historian Hysteria | [Here](./2024/solutions/day01.rs) | ⭐⭐ | 1ms | 19µs | 672µs |
| [Day 2:](https://adventofcode.com/2024/day/2) Red-Nosed Reports | [Here](./2024/solutions/day02.rs) | ⭐⭐ | 1ms | 175µs | 763µs |
| [Day 3:](https://adventofcode.com/2024/day/3) Mull It Over | [Here](./2024/solutions/day03.rs) | ⭐⭐ | 855µs | 8µs | 8µs |

## Run it yourself
### Run a day 
While being at the [2024](./2024/) directory:
```
cargo run -- [--day <day>] [--sample] [--bench <n>]
```
> --day=<1-25> Optional (default:`1`). Specifies the day to execute the puzzle `(1-25)`

> --sample: Optional (default: `false`). If set, the input file will be the `sample.txt` located in the [inputs](./2024/inputs) folder.

> --bench=\<n\>: Optional (default: `0`). If set, the puzzles will run `n` times, outputing only the average timings.

#### Example output
![Output screenshot](https://github.com/user-attachments/assets/072b854a-4e15-4284-a5c5-3745c6bd0f76)
