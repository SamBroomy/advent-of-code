pub mod direction;
pub mod grid;
pub mod point;
#[macro_export]
macro_rules! aoc_run {
    () => {
        fn run() {
            static TEST_INPUT: &str = include_str!("data/test-input.txt");
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
    ($op_1_test: expr, $op_1: expr, $op_2_test: expr, $op_2: expr) => {
        #[cfg(test)]
        mod tests {
            static TEST_INPUT: &str = include_str!("data/test-input.txt");
            static FULL_INPUT: &str = include_str!("data/input.txt");

            #[test]
            fn part_1_test() {
                let output = super::part1(TEST_INPUT);
                assert_eq!(output, $op_1_test);
            }

            #[test]
            fn part_1_real() {
                let output = super::part1(FULL_INPUT);
                assert_eq!(output, $op_1);
            }

            #[test]
            fn part_2_test() {
                let output = super::part2(TEST_INPUT);
                assert_eq!(output, $op_2_test);
            }

            #[test]
            fn part_2_real() {
                let output = super::part2(FULL_INPUT);
                assert_eq!(output, $op_2);
            }
        }
    };
}
