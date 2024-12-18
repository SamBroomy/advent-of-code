use anyhow::Result;
use common::get_input;
use std::collections::VecDeque;

trait Operation {
    fn iter() -> impl Iterator<Item = Self>
    where
        Self: Sized;
    fn operate(&self, a: u64, b: u64) -> u64;
}

#[derive(Debug, Clone, Copy)]
enum OperationP1 {
    Add,
    Multiply,
}

impl Operation for OperationP1 {
    fn iter() -> impl Iterator<Item = OperationP1> {
        const ALLOPERATIONS: [OperationP1; 2] = [OperationP1::Add, OperationP1::Multiply];
        ALLOPERATIONS.iter().copied()
    }

    fn operate(&self, a: u64, b: u64) -> u64 {
        match self {
            OperationP1::Add => a + b,
            OperationP1::Multiply => a * b,
        }
    }
}

#[derive(Debug, Clone)]
struct Question {
    answer: u64,
    inputs: VecDeque<u64>,
}

impl Question {
    fn from_line(line: &str) -> Option<Self> {
        line.split_once(":").and_then(|(answer, qs)| {
            let answer = answer.trim().parse().ok()?;
            let inputs = qs
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect::<VecDeque<u64>>();
            Some(Question { answer, inputs })
        })
    }

    fn solve<O: Operation>(&mut self, acc: u64, mut nums: VecDeque<u64>) -> bool {
        if nums.is_empty() {
            return acc == self.answer;
        }

        let b = nums.pop_front().unwrap();
        O::iter().any(|op| self.solve::<O>(op.operate(acc, b), nums.clone()))
    }

    fn validate<O: Operation>(&mut self) -> Option<u64> {
        let acc = self.inputs.pop_front()?;

        match self.solve::<O>(acc, self.inputs.clone()) {
            true => Some(self.answer),
            false => None,
        }
    }
}

fn parse_input(input: &str) -> Vec<Question> {
    input
        .lines()
        .filter_map(Question::from_line)
        .collect::<Vec<Question>>()
}

fn calculate<O: Operation>(questions: Vec<Question>) -> u64 {
    let (tx, rx) = std::sync::mpsc::channel();
    for mut q in questions.into_iter() {
        let tx = tx.clone();
        std::thread::spawn(move || {
            if let Some(i) = q.validate::<O>() {
                tx.send(i).unwrap();
            }
        });
    }
    drop(tx);
    let mut total = 0;
    while let Ok(i) = rx.recv() {
        total += i;
    }
    total
}

fn part_1(input: &str) -> u64 {
    let questions = parse_input(input);
    calculate::<OperationP1>(questions)
}

#[derive(Debug, Clone, Copy)]
enum OperationP2 {
    Add,
    Multiply,
    Concatenate,
}

impl Operation for OperationP2 {
    fn iter() -> impl Iterator<Item = OperationP2> {
        const ALLOPERATIONS: [OperationP2; 3] = [
            OperationP2::Add,
            OperationP2::Multiply,
            OperationP2::Concatenate,
        ];
        ALLOPERATIONS.iter().copied()
    }

    fn operate(&self, a: u64, b: u64) -> u64 {
        match self {
            OperationP2::Add => a + b,
            OperationP2::Multiply => a * b,
            OperationP2::Concatenate => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

fn part_2(input: &str) -> u64 {
    let questions = parse_input(input);
    calculate::<OperationP2>(questions)
}

fn main() -> Result<()> {
    let input = get_input(7)?;
    let p1 = part_1(&input);
    println!("Part1: {}", p1);

    let p2 = part_2(&input);
    println!("Part2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20";
        let total = part_1(input);
        assert_eq!(total, 3749);
    }

    #[test]
    fn test_part_1() {
        let input = get_input(7).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 20665830408335);
    }

    #[test]
    fn part_2_example() {
        let input = "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20";
        let total = part_2(input);
        assert_eq!(total, 11387);
    }

    #[test]
    fn test_part_2() {
        let input = get_input(7).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 354060705047464);
    }
}
