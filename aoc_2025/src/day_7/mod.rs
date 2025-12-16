use std::str::Lines;

fn parse(input: &str) -> Lines<'_> {
    input.lines()
}

#[inline]
pub fn part1(input: &str) -> u64 {
    // Every other line is blank or contains just `....` so skip it.
    let mut lines = parse(input).step_by(2).map(|l| l.trim());

    let first_line = lines.next().expect("valid input");

    let line_length = first_line.chars().count();

    let mut map = vec![false; line_length];

    let init_pos = first_line
        .chars()
        .position(|c| 'S' == c)
        .expect("start char exists");
    map[init_pos] = true;
    let mut splits_total = 0;

    for line in lines {
        // Find all position of all `^`.
        // if that position matches a true in map, add to the amounts of splits
        // then split by turning that item in the mask to false and the items either side to true.
        let splits = line
            .char_indices()
            .filter_map(|(idx, char)| if '^' == char { Some(idx) } else { None });

        for idx in splits {
            if map[idx] {
                splits_total += 1;
                map[idx] = false;
                // should check for oob but input does not have splitter at the ends of the input so we fine
                map[idx - 1] = true;
                map[idx + 1] = true;
            }
        }
    }

    splits_total
}

#[inline]
pub fn part2(input: &str) -> u64 {
    // Every other line is blank or contains just `....` so skip it.
    let mut lines = parse(input).step_by(2).map(|l| l.trim());

    let first_line = lines.next().expect("valid input");

    let line_length = first_line.chars().count();

    let mut timelines_map = vec![0; line_length];

    let init_pos = first_line
        .chars()
        .position(|c| 'S' == c)
        .expect("start char exists");
    timelines_map[init_pos] = 1;

    for line in lines {
        // Find all position of all `^`.
        // if that position matches a true in map, add to the amounts of splits
        // then split by turning that item in the mask to false and the items either side to true.
        let splits = line
            .char_indices()
            .filter_map(|(idx, char)| if '^' == char { Some(idx) } else { None });

        for idx in splits {
            if timelines_map[idx] > 0 {
                timelines_map[idx - 1] += timelines_map[idx];
                timelines_map[idx + 1] += timelines_map[idx];
                timelines_map[idx] = 0;
            }
        }
    }
    dbg!(&timelines_map);

    timelines_map.iter().sum::<usize>() as u64
}

common::aoc_test!(21, 1590, 40, 20571740188555);
