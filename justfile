AOC_YEAR := env_var_or_default("AOC_YEAR", "2024")

# Print all commands
default:
    @echo "Advent of Code {{AOC_YEAR}}"
    @just --list

new DAY:
    @cargo run --bin aoc_utils fetch --day {{DAY}} --year {{AOC_YEAR}}

next:
    @cargo run --bin aoc_utils next --year {{AOC_YEAR}}

refresh:
    cargo run --bin aoc_utils refresh --year {{AOC_YEAR}}

# Run benchmarks for the year or a specific day
bench DAY="":
    #!/usr/bin/env sh
    if [ "{{DAY}}" = "" ]; then
        cargo bench -- aoc_{{AOC_YEAR}}
        # Run benches/src/main.rs to generate README.md based on benchmark results
        cargo run --bin advent-benches
    else
        # The extra space after day{{DAY}} is needed to avoid `just bench 1` matching with 11, 12...
        cargo bench -- "aoc_{{AOC_YEAR}} day_{{DAY}} "
    fi


# Run tests for the year or a specific day, optionally targeting a specific part
test DAY="" TARGET="":
    #!/usr/bin/env sh
    if [ "{{DAY}}" = "" ]; then
        cargo test --manifest-path aoc_{{AOC_YEAR}}/Cargo.toml
    else
        cargo test --manifest-path aoc_{{AOC_YEAR}}/Cargo.toml -- day_{{DAY}}::tests::part_{{TARGET}} --nocapture
    fi