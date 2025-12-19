use criterion::{criterion_group, criterion_main, Criterion};
use paste::paste;
use std::hint::black_box;

// Macro to generate benchmarks for a specific year
macro_rules! benchmark_year {
    ($year:ident, $($day:ident),+) => {
        paste! {
            $(
                static [<$year:upper _ $day:upper _INPUT>]: &str = include_str!(
                    concat!("../../", stringify!($year), "/src/", stringify!($day), "/data/input.txt")
                );

                fn [<$year _ $day>](c: &mut Criterion) {
                    c.bench_function(
                        &format!("{} {} Part 1", stringify!($year), stringify!($day)),
                        |b| {
                            b.iter(||
                                $year::$day::part1(black_box([<$year:upper _ $day:upper _INPUT>]))
                            );
                        }
                    );
                    c.bench_function(
                        &format!("{} {} Part 2", stringify!($year), stringify!($day)),
                        |b| {
                            b.iter(||
                                $year::$day::part2(black_box([<$year:upper _ $day:upper _INPUT>]))
                            );
                        }
                    );
                }
            )+
        }
    }
}

macro_rules! benchmarks {
    // Match one or more year-days groups
    ($($year:ident {$($day:ident),+ $(,)?}),+ $(,)?) => {
        paste! {
            // For each year-day combination, generate the benchmark
            $(
                $(
                    benchmark_year!{$year, $day}
                )+
            )+

            // Create a single criterion group with all benchmarks
            criterion_group!(
                benches,
                $(
                    $(
                        [<$year _ $day>],
                    )+
                )+
            );
            criterion_main!(benches);
        }
    };
}

benchmarks! {
    aoc_2024 {
        day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, day_10, day_11, day_12, day_13, day_14,
    },
    aoc_2025 {
        day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, day_10, day_11,
    },
}
