use std::collections::VecDeque;

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = u64> + '_,
    impl Iterator<Item = (u64, u64)> + '_,
) {
    let split = input.find("\n\n").unwrap() + '\n'.len_utf8();

    let (ranges, ids) = input.split_at(split);

    let ids = &ids['\n'.len_utf8()..]; // remove leading newline

    let ranges = ranges.lines().map(|line| {
        let mut parts = line.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        (start, end)
    });
    let ids = ids.lines().map(|line| line.parse().unwrap());
    (ids, ranges)
}

#[inline]
pub fn part1(input: &str) -> u64 {
    let (ids, ranges) = parse(input);
    let ranges = ranges.collect::<Vec<_>>();

    ids.filter(|&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count() as u64
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let (_ids, ranges) = parse(input);

    let mut ranges = ranges.collect::<Vec<_>>();

    ranges.sort_unstable_by(|(a_start, a_end), (b_start, b_end)| {
        a_start.cmp(b_start).then(a_end.cmp(b_end))
    });
    let mut ranges = ranges.into_iter().collect::<VecDeque<_>>();

    let mut new_ranges: Vec<(u64, u64)> = vec![];
    let (mut start, mut end) = ranges.pop_front().unwrap();

    while let Some((next_start, next_end)) = ranges.pop_front() {
        // extend range
        if next_start <= end + 1 {
            end = end.max(next_end);
        }
        // complete range
        else {
            new_ranges.push((start, end));
            start = next_start;
            end = next_end;
        }
    }
    new_ranges.push((start, end));

    new_ranges.iter().map(|(start, end)| 1 + end - start).sum()
}

common::aoc_test!(3, 529, 14, 344260049617193);
