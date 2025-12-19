#[derive(Debug)]
enum Sign {
    Add,
    Mult,
}

fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<Sign>) {
    let mut lines = input
        .split('\n')
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let signs = lines
        .pop()
        .unwrap()
        .into_iter()
        .map(|sign: &str| match sign {
            "+" => Sign::Add,
            "*" => Sign::Mult,
            _ => panic!("Invalid character in input"),
        })
        .collect::<Vec<Sign>>();

    (
        lines
            .into_iter()
            .map(|line| line.into_iter().map(|num| num.parse().unwrap()).collect())
            .collect(),
        signs,
    )
}

#[inline]
pub fn part1(input: &str) -> u64 {
    let (numbers, signs) = parse(input);

    let mut total = 0;

    for i in 0..signs.len() {
        let nums = numbers.iter().map(|n| n[i]);
        let problem: u64 = match signs[i] {
            Sign::Add => nums.sum(),
            Sign::Mult => nums.product(),
        };
        total += problem;
    }
    total
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let operator_idx = input.find(['*', '+']).unwrap();

    let num_str = input[..operator_idx].trim_matches('\n');

    let num_str = num_str.split('\n').collect::<Vec<_>>();

    let op_str = &input[operator_idx..].trim_matches('\n');

    let mut op_vec = op_str
        .char_indices()
        .rev()
        .filter(|(_idx, c)| !c.is_whitespace())
        .scan(op_str.len(), |last_idx, (idx, c)| {
            let c = match c {
                '+' => Sign::Add,
                '*' => Sign::Mult,
                _ => panic!("Invalid character in input"),
            };

            let res = Some((c, idx, *last_idx - 1));
            *last_idx = idx;
            res
        })
        .collect::<Vec<_>>();
    // because the last line (reversed) may not have the appropriate whitespace after it
    // so we just need to set it to the longest line length
    op_vec.first_mut().expect("Not an empty iter").2 = num_str
        .iter()
        .map(|s| s.len())
        .max()
        .expect("Not an empty iter"); //op_str.len();

    let mut problem_total = 0u64;
    for (c, idx, distance) in op_vec.into_iter().rev() {
        let mut nums = vec![];
        for i in (0..distance - idx).rev() {
            let mut num = 0;
            let mut place = 0;

            for line in num_str.iter().rev() {
                if let Some(j) = line.chars().nth(idx + i) {
                    if j.is_whitespace() {
                        continue;
                    }
                    num += u64::from(j.to_digit(10).unwrap()) * 10u64.pow(place);
                    place += 1;
                }
            }
            nums.push(num);
        }
        let problem: u64 = match c {
            Sign::Add => nums.iter().sum(),
            Sign::Mult => nums.iter().product(),
        };
        problem_total += problem;
    }
    problem_total
}

common::aoc_test!(4277556, 6171290547579, 3263827, 8811937976367);
