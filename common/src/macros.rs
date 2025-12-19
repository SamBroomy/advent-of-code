#[macro_export]
macro_rules! aoc_run {
    () => {
        fn run() {
            static TEST_INPUT: &str = include_str!("data/sample-input.txt");
            static FULL_INPUT: &str = include_str!("data/input.txt");

            let test_output = part1(TEST_INPUT);
            println!("Test output: {}", test_output);
            let real_output = part1(FULL_INPUT);
            println!("Real output: {}", real_output);
        }
    };
}

#[macro_export]
macro_rules! aoc_test {
    ($op_1_sample: expr, $op_1: expr, $op_2_sample: expr, $op_2: expr) => {
        #[cfg(test)]
        mod tests {
            use std::path::{Path, PathBuf};
            use std::sync::LazyLock;

            static INPUT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
                let caller_file = Path::new(file!());
                let caller_rel = caller_file
                    .strip_prefix(Path::new(env!("CARGO_PKG_NAME")))
                    .unwrap_or(caller_file);
                let caller_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(caller_rel)
                    .parent()
                    .expect("caller file should have a parent")
                    .to_path_buf();
                caller_dir.join("data")
            });

            static SAMPLE_INPUT: &str = include_str!("data/sample-input.txt");

            static TEST_INPUT: LazyLock<String> = LazyLock::new(|| {
                let path = INPUT_DIR.join("input.txt");
                std::fs::read_to_string(path).expect("should read test input")
            });

            #[test]
            fn part_1_sample() {
                let output = super::part1(&SAMPLE_INPUT);
                assert_eq!(output, $op_1_sample);
            }

            #[test]
            #[ignore]
            fn part_1_test() {
                let output = super::part1(&TEST_INPUT);
                assert_eq!(output, $op_1);
            }

            #[test]
            fn part_2_sample() {
                // If a secondary sample file exists next to the calling module's file
                // use it for the part 2 test, otherwise fall back to SAMPLE_INPUT.
                let test_2_sample_input: String = {
                    let alt_input = INPUT_DIR.join("test-input-part-2.txt");
                    if alt_input.exists() {
                        println!(
                            "Using alternate test input for part 2 from: {}",
                            alt_input.display()
                        );
                        std::fs::read_to_string(alt_input)
                            .expect("should read alternate test input")
                    } else {
                        SAMPLE_INPUT.to_string()
                    }
                };

                let output = super::part2(&test_2_sample_input);
                assert_eq!(output, $op_2_sample);
            }

            #[test]
            #[ignore]
            fn part_2_real() {
                let output = super::part2(&TEST_INPUT);
                assert_eq!(output, $op_2);
            }
        }
    };
}
