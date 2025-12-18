use core::slice;
use std::{cmp::Reverse, collections::HashSet};

use kdtree::{KdTree, distance::squared_euclidean};

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    /// Unites the sets containing `x` and `y`.
    ///
    /// Returns `true` if the sets were separate and have now been merged,
    /// or `false` if `x` and `y` were already in the same set.
    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] < self.rank[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.parent[root_y] = root_x;
        if self.rank[root_x] == self.rank[root_y] {
            self.rank[root_x] += 1;
        }
        true
    }

    fn groups(&mut self) -> Vec<Vec<usize>> {
        use std::collections::HashMap;

        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for idx in 0..self.parent.len() {
            let root = self.find(idx);
            map.entry(root).or_default().push(idx);
        }
        map.into_values().collect()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl AsRef<[f64]> for Point {
    fn as_ref(&self) -> &[f64] {
        // SAFETY:
        // - `self` is a valid reference, so `&self.x` is a valid, non-null pointer.
        // - The three `f64` fields are guaranteed to be contiguous in memory.
        // - The lifetime of the returned slice is tied to `&self`, ensuring the data lives long enough.
        unsafe { slice::from_raw_parts(&self.x, 3) }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Point> + '_ {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_terminator(',')
                .take(3)
                .map(|s| s.parse().expect("str should be a valid u32"))
        })
        .map(|mut line| {
            let x = line.next().expect("element should exist");
            let y = line.next().expect("element should exist");
            let z = line.next().expect("element should exist");

            Point { x, y, z }
        })
}

#[inline]
pub fn part1(input: &str) -> u64 {
    let dimensions = 3;
    let mut kdtree = KdTree::new(dimensions);

    let points = parse(input).enumerate().collect::<Vec<_>>();
    for (idx, p) in &points {
        kdtree.add(p, idx).expect("Valid input");
    }

    let mut uf = UnionFind::new(points.len());
    let mut nearest_pairs: Vec<(f64, usize, usize)> = Vec::with_capacity(points.len() * 2);
    let mut seen_pairs: HashSet<(usize, usize)> = HashSet::with_capacity(points.len() * 2);
    let cap = if points.len() > 100 { 1000 } else { 10 };
    let mut max_distance = f64::MAX;

    for (idx, p) in &points {
        // iter over nearest till we hit the max distance, first iteration we will get all points.
        for (distance, nearest_idx) in kdtree.iter_nearest(p.as_ref(), &squared_euclidean).unwrap()
        {
            if distance > max_distance {
                break;
            }
            if **nearest_idx == *idx {
                continue;
            }
            let (a, b) = if *idx < **nearest_idx {
                (*idx, **nearest_idx)
            } else {
                (**nearest_idx, *idx)
            };
            if !seen_pairs.insert((a, b)) {
                continue;
            }
            nearest_pairs.push((distance, a, b));
        }

        nearest_pairs.sort_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).unwrap());
        if nearest_pairs.len() > cap {
            nearest_pairs.truncate(cap);
        }
        if nearest_pairs.len() == cap {
            max_distance = nearest_pairs.last().unwrap().0;
        }
    }
    for (_, id, nearest_idx) in nearest_pairs.iter() {
        uf.union(*id, *nearest_idx);
    }

    let mut groups = uf.groups();
    groups.sort_by_key(|g| Reverse(g.len()));
    u64::try_from(groups.iter().take(3).map(|g| g.len()).product::<usize>()).expect("no overflow")
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let dimensions = 3;
    let mut kdtree = KdTree::new(dimensions);

    let points = parse(input).enumerate().collect::<Vec<_>>();
    for (idx, p) in &points {
        kdtree.add(p, idx).expect("Valid input");
    }

    let mut uf = UnionFind::new(points.len());
    let mut edges = Vec::new();
    let mut seen_pairs: HashSet<(usize, usize)> = HashSet::new();

    let mut components = points.len();

    for (idx, p) in &points {
        // iter over nearest till we hit the max distance, first iteration we will get all points.
        for (distance, nearest_idx) in kdtree.iter_nearest(p.as_ref(), &squared_euclidean).unwrap()
        {
            if **nearest_idx == *idx {
                continue;
            }
            let (a, b) = if *idx < **nearest_idx {
                (*idx, **nearest_idx)
            } else {
                (**nearest_idx, *idx)
            };
            if !seen_pairs.insert((a, b)) {
                continue;
            }
            edges.push((distance, a, b));
        }
    }
    edges.sort_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).unwrap());

    for (_dist, id, nearest_idx) in &edges {
        if uf.union(*id, *nearest_idx) {
            components -= 1;
            if components == 1 {
                println!("Connected all components");
                println!(
                    "Last edge: {:?} <-> {:?}",
                    points[*id], points[*nearest_idx]
                );
                return points[*id].1.x as u64 * points[*nearest_idx].1.x as u64;
            }
        }
    }
    unreachable!("Could not connect all components");
}

common::aoc_test!(40, 123234, 25272, 9259958565);
