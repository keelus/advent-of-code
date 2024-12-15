# Advent of Code 2024 🎄🎁
This repo contains my solutions for different years.
- 2023 solutions in Golang [here](./2023/).
- 2024 solutions in Rust [here](./2024/).

## Days, stars and solutions 
> Benchmarks are run on an "Intel i7-10700K @ 5.100Ghz CPU".
> Use the `--release` flag when benchmarking a day.

| Day | Solution | Stars | Parse benchmark | Part 1 benchmark | Part 2 benchmark |
|-----|----------|-------|-------------------|-----------------|------------------|
| [Day 1:](https://adventofcode.com/2024/day/1) Historian Hysteria | [Here](./2024/solutions/day01.rs) | ⭐⭐ | 63µ | 264ns | 50µs |
| [Day 2:](https://adventofcode.com/2024/day/2) Red-Nosed Reports | [Here](./2024/solutions/day02.rs) | ⭐⭐ | 142µs | 7µs | 57µs |
| [Day 3:](https://adventofcode.com/2024/day/3) Mull It Over | [Here](./2024/solutions/day03.rs) | ⭐⭐ | 65µs | 345ns | 466ns |
| [Day 4:](https://adventofcode.com/2024/day/4) Ceres Search | [Here](./2024/solutions/day04.rs) | ⭐⭐ | 7µs | 1ms | 360µs |
| [Day 5:](https://adventofcode.com/2024/day/5) Print Queue | [Here](./2024/solutions/day05.rs) | ⭐⭐ | 201µs | 395µs | 797µs |
| [Day 6:](https://adventofcode.com/2024/day/6) Guard Gallivant | [Here](./2024/solutions/day06.rs) | ⭐⭐ | 36µs | 251µs | 1s |
| [Day 7:](https://adventofcode.com/2024/day/7) Bridge Repair | [Here](./2024/solutions/day07.rs) | ⭐⭐ | 455µs | 578µs | 84ms |
| [Day 8:](https://adventofcode.com/2024/day/8) Resonant Collinearity | [Here](./2024/solutions/day08.rs) | ⭐⭐ | 21µs | 116µs | 92µs |
| [Day 9:](https://adventofcode.com/2024/day/9) Disk Fragmenter | [Here](./2024/solutions/day09.rs) | ⭐⭐ | 1ms | 520ms | 332ms |
| [Day 10:](https://adventofcode.com/2024/day/10) Hoof It | [Here](./2024/solutions/day10.rs) | ⭐⭐ | 65µs | 489µs | 436µs |
| [Day 11:](https://adventofcode.com/2024/day/11) Plutonian Pebbles | [Here](./2024/solutions/day11.rs) | ⭐⭐ | 1µs | 582µs | 32ms |
| [Day 12:](https://adventofcode.com/2024/day/12) Garden Groups | [Here](./2024/solutions/day12.rs) | ⭐⭐ | 221µs | 7ms | 11ms |
| [Day 13:](https://adventofcode.com/2024/day/13) Claw Contraption | [Here](./2024/solutions/day13.rs) | ⭐⭐ | 158µs | 3µs | 6µs |
| [Day 14:](https://adventofcode.com/2024/day/14) Restroom Redoubt | [Here](./2024/solutions/day14.rs) | ⭐⭐ | 94µs | 180µs | 790ms |
| [Day 15:](https://adventofcode.com/2024/day/15) Warehouse Woes | [Here](./2024/solutions/day15.rs) | ⭐⭐ | 210µs | 130µs | 3ms |

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
