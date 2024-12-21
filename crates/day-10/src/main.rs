use anyhow::Result;
use common::get_input;
use std::fmt::Display;
use tree_ds::prelude::*;

type Grid = Vec<Vec<u8>>;
type MapNode = Node<Coordinate, u8>;
type MapTree = Tree<Coordinate, u8>;

/// Represents a 2D coordinate in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    i: usize,
    j: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.i, self.j)
    }
}

impl Coordinate {
    fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    fn get_next_coordinates(&self) -> [Option<Coordinate>; 4] {
        let mut next_coords = [None; 4];

        let i = self.i;
        let j = self.j;
        if let Some(i) = i.checked_sub(1) {
            next_coords[0] = Some(Coordinate::new(i, j));
        }
        if let Some(i) = i.checked_add(1) {
            next_coords[1] = Some(Coordinate::new(i, j));
        }
        if let Some(j) = j.checked_sub(1) {
            next_coords[2] = Some(Coordinate::new(i, j));
        }
        if let Some(j) = j.checked_add(1) {
            next_coords[3] = Some(Coordinate::new(i, j));
        }
        next_coords
    }
}

/// Extension traits for MapNode operations
trait MapNodeExt {
    fn get_next(&self, value: &u8, next: Coordinate, grid: &Grid) -> Option<MapNode>;
    fn get_next_coordinates(&self, grid: &Grid) -> Vec<MapNode>;
}

impl MapNodeExt for MapNode {
    fn get_next(&self, value: &u8, next: Coordinate, grid: &Grid) -> Option<MapNode> {
        grid.get(next.i)
            .and_then(|v| v.get(next.j))
            .and_then(|&val| {
                if val == (value + 1) {
                    Some(Node::new(next, Some(val)))
                } else {
                    None
                }
            })
    }

    fn get_next_coordinates(&self, grid: &Grid) -> Vec<MapNode> {
        let mut next_nodes = Vec::with_capacity(4);
        let value = self.get_value().unwrap();
        for next in self
            .get_node_id()
            .get_next_coordinates()
            .into_iter()
            .flatten()
        {
            if let Some(node) = self.get_next(&value, next, grid) {
                next_nodes.push(node);
            }
        }
        next_nodes
    }
}

// Tree building functions
fn get_unique_tree(grid: &Grid, root_node: MapNode, name: impl Into<Option<String>>) -> MapTree {
    let mut tree = Tree::new(name.into().as_deref());
    let children_nodes = root_node.get_next_coordinates(grid);
    let id = tree.add_node(root_node, None).unwrap();
    for child in children_nodes {
        add_node_to_tree_unique(grid, &mut tree, id, child);
    }
    tree
}

fn add_node_to_tree_unique(grid: &Grid, tree: &mut MapTree, parent_id: Coordinate, child: MapNode) {
    if tree.get_node_by_id(&child.get_node_id()).is_some() {
        return;
    }
    let children_nodes = child.get_next_coordinates(grid);
    let val = child.get_value().unwrap();
    let id = tree.add_node(child, Some(&parent_id)).unwrap();
    if val == 9 {
        return;
    }
    for child in children_nodes {
        add_node_to_tree_unique(grid, tree, id, child);
    }
}

fn get_tree(grid: &Grid, root_node: MapNode, name: impl Into<Option<String>>) -> MapTree {
    let mut tree = Tree::new(name.into().as_deref());
    let children_nodes = root_node.get_next_coordinates(grid);
    let id = tree.add_node(root_node, None).unwrap();
    for child in children_nodes {
        add_node_to_tree(grid, &mut tree, id, child);
    }
    tree
}

fn add_node_to_tree(grid: &Grid, tree: &mut MapTree, parent_id: Coordinate, child: MapNode) {
    // Check if this exact child already exists under this parent
    if let Some(parent) = tree.get_node_by_id(&parent_id) {
        if parent.get_children_ids().iter().any(|child_id| {
            tree.get_node_by_id(child_id)
                .map(|n| n.get_node_id() == child.get_node_id())
                .unwrap_or(false)
        }) {
            return;
        }
    }

    let children_nodes = child.get_next_coordinates(grid);

    let val = child.get_value().unwrap();
    let id = tree.add_node(child, Some(&parent_id)).unwrap();
    if val == 9 {
        return;
    }
    for grand_child in children_nodes {
        add_node_to_tree(grid, tree, id, grand_child);
    }
}

// Path calculation functions
fn get_unique_paths(tree: &MapTree) -> usize {
    tree.get_nodes()
        .iter()
        .filter(|node| node.get_value() == Some(9))
        .count()
}

fn get_total_paths(tree: &MapTree) -> usize {
    let root = tree.get_root_node().unwrap();
    root.get_children_ids()
        .iter()
        .map(|id| get_total_paths_recursive(tree, *id))
        .sum::<usize>()
}

fn get_total_paths_recursive(tree: &MapTree, id: Coordinate) -> usize {
    let node = tree.get_node_by_id(&id).unwrap();
    if node.get_value() == Some(9) {
        return 1;
    }
    node.get_children_ids()
        .iter()
        .map(|id| get_total_paths_recursive(tree, *id))
        .sum::<usize>()
}

// Input parsing and processing
fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap_or(99) as u8)
                .collect()
        })
        .collect()
}

fn get_root_nodes(grid: &Grid) -> Vec<MapNode> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &val)| {
                if val == 0 {
                    Some(Node::new(Coordinate::new(i, j), Some(val)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<MapNode>>()
}

// Solution functions
fn part_1(input: &str) -> i32 {
    let grid = parse_input(input);
    let root_nodes = get_root_nodes(&grid);
    root_nodes
        .into_iter()
        .enumerate()
        .map(|(i, node)| get_unique_tree(&grid, node, i.to_string()))
        .map(|tree| get_unique_paths(&tree))
        .sum::<usize>() as i32
}

fn part_2(input: &str) -> i32 {
    let grid = parse_input(input);
    let root_nodes = get_root_nodes(&grid);
    root_nodes
        .into_iter()
        .enumerate()
        .map(|(i, node)| get_tree(&grid, node, i.to_string()))
        .map(|tree| get_total_paths(&tree))
        .sum::<usize>() as i32
}

fn main() -> Result<()> {
    let input = get_input(10)?;

    let start = std::time::Instant::now();
    let p1 = part_1(&input);
    println!("Part 1: {} (took {:?})", p1, start.elapsed());

    let start = std::time::Instant::now();
    let p2 = part_2(&input);
    println!("Part 2: {} (took {:?})", p2, start.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    const EXAMPLE_INPUT_2: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

    const EXAMPLE_INPUT_3: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    const EXAMPLE_INPUT_4: &str = "012345
123456
234567
345678
4.6789
56789.";

    #[test]
    fn part_1_example() {
        let total = part_1(EXAMPLE_INPUT);
        assert_eq!(total, 36);
    }

    #[test]
    fn test_part_1() {
        let input = get_input(10).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 574);
    }

    #[test]
    fn part_2_example() {
        let total = part_2(EXAMPLE_INPUT);
        assert_eq!(total, 81);
    }

    #[test]
    fn part_2_example_2() {
        let total = part_2(EXAMPLE_INPUT_2);
        assert_eq!(total, 3);
    }

    #[test]
    fn part_2_example_3() {
        let total = part_2(EXAMPLE_INPUT_3);
        assert_eq!(total, 13);
    }

    #[test]
    fn part_2_example_4() {
        let total = part_2(EXAMPLE_INPUT_4);
        assert_eq!(total, 227);
    }

    #[test]
    fn test_part_2() {
        let input = get_input(10).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 1238);
    }
}
