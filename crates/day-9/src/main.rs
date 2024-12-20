use core::fmt;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    ops::Add,
};

use anyhow::Result;
use common::get_input;

enum SplitResult {
    Split(File, File, Free),
    Match,
    NoSplit(File, Free, Free),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    id: usize,
    length: u32,
}

impl File {
    fn new(id: usize, length: u32) -> Self {
        if length == 0 {
            panic!("File length must be greater than 0");
        }
        Self { id, length }
    }

    fn split(self, free: Free) -> SplitResult {
        match self.length.cmp(&free.length) {
            Ordering::Greater => SplitResult::Split(
                File::new(self.id, free.length),
                File::new(self.id, self.length - free.length),
                Free::new(free.length),
            ),
            Ordering::Equal => SplitResult::Match,
            Ordering::Less => {
                let free_new_gap = Free::new(free.length - self.length);
                let free_original_gap = Free::new(self.length);
                SplitResult::NoSplit(self, free_new_gap, free_original_gap)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Free {
    length: u32,
}

impl Free {
    fn new(length: u32) -> Self {
        if length == 0 {
            panic!("Free length must be greater than 0");
        }
        Self { length }
    }
}

impl Add for Free {
    type Output = Free;

    fn add(self, rhs: Self) -> Self::Output {
        Free {
            length: self.length + rhs.length,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DiskBlock {
    File(File),
    Free(Free),
}

type Disk = Vec<DiskBlock>;

impl DiskBlock {
    fn from_input(input: &str) -> Disk {
        input
            .trim()
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                let length = c.to_digit(10)?;
                if length == 0 {
                    return None;
                }
                Some(if i % 2 == 0 {
                    DiskBlock::File(File::new(i / 2, length))
                } else {
                    DiskBlock::Free(Free::new(length))
                })
            })
            .collect()
    }

    fn length(&self) -> u32 {
        match self {
            DiskBlock::File(f @ File { .. }) => f.length,
            DiskBlock::Free(f @ Free { .. }) => f.length,
        }
    }
}

impl Display for DiskBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DiskBlock::File(File { id, length }) => {
                write!(f, "{}", format!("{}", id).repeat(*length as usize))
            }
            DiskBlock::Free(Free { length }) => write!(f, "{}", ".".repeat(*length as usize)),
        }
    }
}

fn combine_free_blocks(blocks: &mut Disk) {
    let mut i = 0;
    while i + 1 < blocks.len() {
        if let (DiskBlock::Free(f1), DiskBlock::Free(f2)) = (&blocks[i], &blocks[i + 1]) {
            blocks[i] = DiskBlock::Free(Free {
                length: f1.length + f2.length,
            });
            blocks.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

fn format_blocks(mut blocks: Disk) -> Disk {
    loop {
        let (i, free) = blocks
            .iter()
            .enumerate()
            .find_map(|b| match b.1 {
                DiskBlock::Free(f) => Some((b.0, f.clone())),
                _ => None,
            })
            .unwrap();

        let (j, file) = blocks
            .iter()
            .enumerate()
            .rev()
            .find_map(|b| match b.1 {
                DiskBlock::File(f) => Some((b.0, f.clone())),
                _ => None,
            })
            .unwrap();

        if i >= j {
            break;
        }
        match file.split(free) {
            SplitResult::Match => {
                blocks.swap(i, j);
            }
            SplitResult::NoSplit(fit, free, remaining) => {
                blocks[j] = DiskBlock::Free(remaining);
                blocks.insert(i, DiskBlock::File(fit));
                blocks[i + 1] = DiskBlock::Free(free);
            }
            SplitResult::Split(fit, remaining, free) => {
                blocks[j] = DiskBlock::File(remaining);
                blocks.insert(j, DiskBlock::Free(free));
                blocks[i] = DiskBlock::File(fit);
            }
        }
        combine_free_blocks(&mut blocks);
    }
    blocks
}

fn format_whole_blocks(mut blocks: Disk) -> Disk {
    let mut iter_files = blocks.clone().into_iter().filter_map(|b| match b {
        DiskBlock::File(f) => Some(f),
        _ => None,
    });

    while let Some(file) = iter_files.next_back() {
        if let Some((i, res)) = blocks
            .clone()
            .into_iter()
            .enumerate()
            .find_map(|(j, b)| match b {
                DiskBlock::Free(free) => match file.clone().split(free.clone()) {
                    SplitResult::Match => Some((j, file.clone().split(free.clone()))),
                    SplitResult::NoSplit(_, _, _) => Some((j, file.clone().split(free.clone()))),
                    _ => None,
                },
                _ => None,
            })
        {
            let Some(j) = blocks
                .iter()
                .position(|b| *b == DiskBlock::File(file.clone()))
            else {
                continue;
            };
            if i >= j {
                continue;
            }
            match res {
                SplitResult::Match => {
                    blocks.swap(i, j);
                }
                SplitResult::NoSplit(fit, free, remaining) => {
                    blocks[j] = DiskBlock::Free(remaining);
                    blocks.insert(i, DiskBlock::File(fit));
                    blocks[i + 1] = DiskBlock::Free(free);
                }
                _ => {
                    continue;
                }
            }
        }
        combine_free_blocks(&mut blocks);
    }
    blocks
}

fn calculate_checksum(blocks: &Disk) -> usize {
    let mut checksum = 0;
    let mut ptr = 0;
    blocks.iter().for_each(|b| {
        let len = b.length() as usize;
        if let DiskBlock::File(f) = b {
            let total = (ptr..ptr + len).fold(0, |acc, x| acc + (f.id * x));
            checksum += total;
        }
        ptr += len;
    });
    checksum
}

fn part_1(input: &str) -> usize {
    let blocks = DiskBlock::from_input(input);
    let blocks = format_blocks(blocks);
    calculate_checksum(&blocks)
}

fn part_2(input: &str) -> usize {
    let blocks = DiskBlock::from_input(input);

    let blocks = format_whole_blocks(blocks);

    calculate_checksum(&blocks)
}

fn main() -> Result<()> {
    let input = get_input(9)?;
    let start = std::time::Instant::now();
    let p1 = part_1(&input);
    println!("Time: {:?}", start.elapsed());
    println!("Part1: {}", p1);
    let start = std::time::Instant::now();
    let p2 = part_2(&input);
    println!("Time: {:?}", start.elapsed());
    println!("Part2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "2333133121414131402";
        let total = part_1(input);
        assert_eq!(total, 1928);
    }

    #[test]
    fn test_part_1() {
        let input = get_input(9).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 6398252054886);
    }

    #[test]
    fn part_2_example() {
        let input = "2333133121414131402";
        let total = part_2(input);
        assert_eq!(total, 2858);
    }

    #[test]
    fn test_part_2() {
        let input = get_input(9).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 6415666220005);
    }
}
