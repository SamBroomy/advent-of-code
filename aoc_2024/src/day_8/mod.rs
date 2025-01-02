use anyhow::{Error, Result};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Antenna {
    i: usize,
    j: usize,
}

impl Antenna {
    fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }
}

// Ordering based on (i,j)
impl PartialOrd for Antenna {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Antenna {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.i.cmp(&other.i) {
            Ordering::Equal => self.j.cmp(&other.j),
            ord => ord,
        }
    }
}

#[derive(Debug)]
struct Vector {
    i: i32,
    j: i32,
}

impl Vector {
    fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct AntiNode {
    i: i32,
    j: i32,
}

impl AntiNode {
    fn from_antenna_and_vector(antenna: &Antenna, vector: &Vector) -> Self {
        Self {
            i: antenna.i as i32 + vector.i,
            j: antenna.j as i32 + vector.j,
        }
    }
}

impl TryFrom<&AntiNode> for Antenna {
    type Error = Error;

    fn try_from(an: &AntiNode) -> Result<Self> {
        Ok(Self {
            i: usize::try_from(an.i)?,
            j: usize::try_from(an.j)?,
        })
    }
}

impl From<&Antenna> for AntiNode {
    fn from(antenna: &Antenna) -> Self {
        Self {
            i: antenna.i as i32,
            j: antenna.j as i32,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct AntennaPair {
    a: Antenna,
    b: Antenna,
}

impl AntennaPair {
    fn new(a: Antenna, b: Antenna) -> Self {
        Self { a, b }
    }

    fn get_vector(&self) -> Vector {
        // A vector from a -> b
        Vector::new(
            self.b.i as i32 - self.a.i as i32,
            self.b.j as i32 - self.a.j as i32,
        )
    }

    fn get_antinode(&self) -> AntiNode {
        let v = self.get_vector();
        // from b to anti-node
        AntiNode::from_antenna_and_vector(&self.b, &v)
    }

    fn get_all_antinodes(&self, (i_lim, j_lim): (usize, usize)) -> Vec<AntiNode> {
        let mut all_antinodes = Vec::new();
        all_antinodes.push((&self.b).into());

        let vector = self.get_vector();
        let mut anti_node = self.get_antinode();
        while let Ok(antenna) = Antenna::try_from(&anti_node) {
            if antenna.i >= i_lim || antenna.j >= j_lim {
                break;
            }
            all_antinodes.push(anti_node.clone());
            anti_node = AntiNode::from_antenna_and_vector(&antenna, &vector);
        }
        all_antinodes
    }
}

struct AntennaGrid {
    grid: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<Antenna>>,
}

impl AntennaGrid {
    fn new(grid: Vec<Vec<char>>, antennas: HashMap<char, Vec<Antenna>>) -> Self {
        Self { grid, antennas }
    }

    fn parse_input(input: &str) -> AntennaGrid {
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let mut antennas = HashMap::<char, Vec<Antenna>>::new();

        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                let antenna = Antenna::new(i, j);
                antennas
                    .entry(*c)
                    .and_modify(|e| e.push(antenna.clone()))
                    .or_insert(vec![antenna]);
            }
        }
        AntennaGrid::new(grid, antennas)
    }

    fn get_antenna_combinations_from_slice(antennas: &[Antenna]) -> HashSet<AntennaPair> {
        antennas
            .iter()
            .flat_map(|a| {
                antennas.iter().filter_map(move |b| {
                    if a == b {
                        None
                    } else {
                        Some(AntennaPair::new(a.clone(), b.clone()))
                    }
                })
            })
            .collect()
    }

    fn get_all_antenna_pairs(&self) -> HashMap<char, HashSet<AntennaPair>> {
        self.antennas
            .iter()
            .map(|(k, v)| (*k, Self::get_antenna_combinations_from_slice(v)))
            .collect()
    }

    fn get_all_antinodes(&self) -> Vec<AntiNode> {
        let mut antinodes = Vec::new();
        for antenna_pairs in self.get_all_antenna_pairs().values() {
            for pair in antenna_pairs {
                let a = pair.get_antinode();
                antinodes.push(a);
            }
        }
        antinodes
    }

    fn check_antinodes_on_grid(&self) -> HashSet<AntiNode> {
        let antinodes = self.get_all_antinodes();
        antinodes
            .into_iter()
            .filter(|an| {
                an.i >= 0
                    && an.j >= 0
                    && an.i < self.grid.len() as i32
                    && an.j < self.grid[0].len() as i32
            })
            .collect::<HashSet<_>>()
    }

    fn get_all_propogating_antinodes(&self) -> HashSet<AntiNode> {
        self.get_all_antenna_pairs()
            .values()
            .flat_map(|antenna_pairs| {
                antenna_pairs
                    .iter()
                    .flat_map(|pair| pair.get_all_antinodes((self.grid.len(), self.grid[0].len())))
            })
            .collect()
    }
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let ag = AntennaGrid::parse_input(input);
    let antenna = ag.check_antinodes_on_grid();
    antenna.len() as i32
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let ag = AntennaGrid::parse_input(input);
    let antenna = ag.get_all_propogating_antinodes();
    antenna.len() as i32
}

common::aoc_test!(14, 359, 34, 1293);
