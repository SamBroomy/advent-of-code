fn parse(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.split(',').flat_map(|i| {
        let (first, second) = i.split_once('-').unwrap();
        let first = first.parse().unwrap();
        let second = second.parse().unwrap();
        first..=second
    })
}

// fn check_equal_parts(num: u64, part_len: u32, parts: u32) -> bool {
//     let div = 10_u64.pow(part_len);
//     let first_part = num / div.pow(parts - 1);
//     for i in 1..parts {
//         let current_part = (num / div.pow(parts - 1 - i)) % div;
//         if current_part != first_part {
//             return false;
//         }
//     }
//     true
// }

#[inline]
pub fn part1(input: &str) -> u64 {
    parse(input)
        .filter(|num| {
            let len = if *num == 0 {
                return false;
            } else {
                num.ilog10() + 1
            };
            if len % 2 != 0 {
                return false;
            }
            let half = len / 2;
            let div = 10_u64.pow(half);

            let first = num / div;
            let second = num % div;

            first == second
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u64 {
    parse(input)
        .filter(|num| {
            let num_len = if *num == 0 { 1 } else { num.ilog10() + 1 };
            let mut is_invalid = false;

            for part_len in 1..=(num_len / 2) {
                if num_len % part_len != 0 {
                    continue;
                }
                let first_part = num / 10_u64.pow(num_len - part_len);

                let num_repetitions = num_len / part_len;

                let mut all_match = true;

                for r in 1..num_repetitions {
                    let div = 10_u64.pow(part_len);
                    let current_part = (num / div.pow(num_repetitions - r - 1)) % div;

                    if first_part != current_part {
                        all_match = false;
                        break;
                    }
                }
                if all_match {
                    is_invalid = true;
                    break;
                }
            }
            is_invalid
        })
        .sum()
}

common::aoc_test!(1227775554, 31000881061, 4174379265, 46769308485
);
