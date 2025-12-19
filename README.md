# Advent of Code

## Benchmarks

Successfully completed problems with the time taken to execute them on my M1 Pro.

### 2025

| Day  | Problem     | Solution    | Part 1 (ms) | Part 2 (ms) | Total (ms) |
|------|-------------|-------------|-------------|-------------|------------|
| 1 | [Secret Entrance](https://adventofcode.com/2025/day/1) | [Solution](/aoc_2025/src/day_1/mod.rs) | 0.05 | 0.07 | 0.12 |
| 2 | [Gift Shop](https://adventofcode.com/2025/day/2) | [Solution](/aoc_2025/src/day_2/mod.rs) | 6.43 | 36.98 | 43.41 |
| 3 | [Lobby](https://adventofcode.com/2025/day/3) | [Solution](/aoc_2025/src/day_3/mod.rs) | 0.06 | 0.16 | 0.22 |
| 4 | [Printing Department](https://adventofcode.com/2025/day/4) | [Solution](/aoc_2025/src/day_4/mod.rs) | 0.25 | 4.58 | 4.83 |
| 5 | [Cafeteria](https://adventofcode.com/2025/day/5) | [Solution](/aoc_2025/src/day_5/mod.rs) | 0.11 | 0.01 | 0.12 |
| 6 | [Trash Compactor](https://adventofcode.com/2025/day/6) | [Solution](/aoc_2025/src/day_6/mod.rs) | 0.08 | 1.45 | 1.53 |
| 7 | [Laboratories](https://adventofcode.com/2025/day/7) | [Solution](/aoc_2025/src/day_7/mod.rs) | 0.02 | 0.02 | 0.03 |
| 8 | [Playground](https://adventofcode.com/2025/day/8) | [Solution](/aoc_2025/src/day_8/mod.rs) | 5.41 | 112.00 | 117.40 |
| 9 | [Movie Theater](https://adventofcode.com/2025/day/9) | [Solution](/aoc_2025/src/day_9/mod.rs) | 0.11 | 923.38 | 923.49 |
| 10 | [Factory](https://adventofcode.com/2025/day/10) | [Solution](/aoc_2025/src/day_10/mod.rs) | 1.48 | 249.47 | 250.96 |
| 11 | [Reactor](https://adventofcode.com/2025/day/11) | [Solution](/aoc_2025/src/day_11/mod.rs) | 0.15 | 0.46 | 0.61 |
|  |  | Total | 14.15ms | 1328.58ms | 1342.73ms |

### 2024

| Day  | Problem     | Solution    | Part 1 (ms) | Part 2 (ms) | Total (ms) |
|------|-------------|-------------|-------------|-------------|------------|
| 1 | [Historian Hysteria](https://adventofcode.com/2024/day/1) | [Solution](/aoc_2024/src/day_1/mod.rs) | 0.05 | 0.05 | 0.10 |
| 2 | [Red-Nosed Reports](https://adventofcode.com/2024/day/2) | [Solution](/aoc_2024/src/day_2/mod.rs) | 0.09 | 0.14 | 0.23 |
| 3 | [Mull It Over](https://adventofcode.com/2024/day/3) | [Solution](/aoc_2024/src/day_3/mod.rs) | 0.19 | 0.28 | 0.47 |
| 4 | [Ceres Search](https://adventofcode.com/2024/day/4) | [Solution](/aoc_2024/src/day_4/mod.rs) | 0.38 | 0.36 | 0.74 |
| 5 | [Print Queue](https://adventofcode.com/2024/day/5) | [Solution](/aoc_2024/src/day_5/mod.rs) | 0.13 | 0.16 | 0.29 |
| 6 | [Guard Gallivant](https://adventofcode.com/2024/day/6) | [Solution](/aoc_2024/src/day_6/mod.rs) | 0.23 | 113.62 | 113.85 |
| 7 | [Bridge Repair](https://adventofcode.com/2024/day/7) | [Solution](/aoc_2024/src/day_7/mod.rs) | 10.87 | 176.19 | 187.06 |
| 8 | [Resonant Collinearity](https://adventofcode.com/2024/day/8) | [Solution](/aoc_2024/src/day_8/mod.rs) | 0.07 | 0.15 | 0.22 |
| 9 | [Disk Fragmenter](https://adventofcode.com/2024/day/9) | [Solution](/aoc_2024/src/day_9/mod.rs) | 142.20 | 293.90 | 436.10 |
| 10 | [Hoof It](https://adventofcode.com/2024/day/10) | [Solution](/aoc_2024/src/day_10/mod.rs) | 0.71 | 1.06 | 1.77 |
| 11 | [Plutonian Pebbles](https://adventofcode.com/2024/day/11) | [Solution](/aoc_2024/src/day_11/mod.rs) | 0.11 | 5.67 | 5.78 |
| 12 | [Garden Groups](https://adventofcode.com/2024/day/12) | [Solution](/aoc_2024/src/day_12/mod.rs) | 2.37 | 3.25 | 5.62 |
| 13 | [Claw Contraption](https://adventofcode.com/2024/day/13) | [Solution](/aoc_2024/src/day_13/mod.rs) | 0.16 | 0.16 | 0.31 |
| 14 | [Restroom Redoubt](https://adventofcode.com/2024/day/14) | [Solution](/aoc_2024/src/day_14/mod.rs) | 0.15 | 0.22 | 0.37 |
|  |  | Total | 157.71ms | 595.21ms | 752.91ms |

## How to use

Run `just` for all the available commands.

By default `just` will run these for the latest year, set by the env variable `AOC_YEAR`.

```
just fetch 15         # fetches the 15th day's problem and input.

just test 15 1_t      # runs day15::tests::part1_test
just test 15 1_r      # runs day15::tests::part1_real
just test 15 1        # runs both tests for day 15 part 1
just test 15 2        # runs both tests for day 15 part 2
just test 15          # runs all 4 tests for day 15

just submit 15 1 1024 # Submit "1024" as the solution for Day 15 Part 1
just submit 15 2 2048 # Submit "2048" as the solution for Day 15 Part 2

just bench 15         # benchmarks day 15 parts 1 and 2
```

`just test`/`just bench` with no arguments runs all the tests/benchmarks for the latest year.

### Overriding `AOC_YEAR`

If `AOC_YEAR` is not set, it picks up the default from the `justfile`. To run the commands for a different year, you can choose one of these options:

- Set it permanently
  - Set the env variable - `export AOC_YEAR=2023`
  - Change the default in the `justfile` - `AOC_YEAR := env_var_or_default("AOC_YEAR", "2023")`
- Set it for one invocation
  - `AOC_YEAR=2023 just test` OR
  - `just --set AOC_YEAR 2023 test`

## Disclaimer

Most of the benchmarking has come from `nindalf's` [advent of code repo](https://github.com/nindalf/advent). I have learnt a lot from his code and have used some of his benchmarks to compare my solutions. I have also used his `justfile` as a base for mine. I have made some modifications to suit my needs
