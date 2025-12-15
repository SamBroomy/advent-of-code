enum PaperRoll {
    Empty,
    Full,
}

fn parse(input: &str) -> Vec<Vec<PaperRoll>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => PaperRoll::Empty,
                    '@' => PaperRoll::Full,
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let input = parse(input);
    let mut count = 0;
    for i in 0..input.len() {
        'j: for j in 0..input[0].len() {
            if matches!(input[i][j], PaperRoll::Empty) {
                continue;
            }
            let mut inner_count = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }

                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0
                        && ni < input.len() as isize
                        && nj >= 0
                        && nj < input[0].len() as isize
                        && matches!(input[ni as usize][nj as usize], PaperRoll::Full)
                    {
                        inner_count += 1;
                        if inner_count >= 4 {
                            continue 'j;
                        }
                    }
                }
            }
            count += 1;
        }
    }

    count
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let mut input = parse(input);
    let mut removed_count = 0;
    let mut to_be_removed = vec![];

    loop {
        for i in 0..input.len() {
            'j: for j in 0..input[0].len() {
                if matches!(input[i][j], PaperRoll::Empty) {
                    continue;
                }
                let mut inner_count = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }

                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0
                            && ni < input.len() as isize
                            && nj >= 0
                            && nj < input[0].len() as isize
                            && matches!(input[ni as usize][nj as usize], PaperRoll::Full)
                        {
                            inner_count += 1;
                            if inner_count >= 4 {
                                continue 'j;
                            }
                        }
                    }
                }
                to_be_removed.push((i, j));
            }
        }
        if to_be_removed.is_empty() {
            break;
        }
        removed_count += to_be_removed.len() as i32;

        for (i, j) in to_be_removed.drain(..) {
            input[i][j] = PaperRoll::Empty;
        }
    }

    removed_count
}

common::aoc_test!(13, 1445, 43, 8317);
