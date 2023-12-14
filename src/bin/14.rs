use std::fmt::Write;
use ndarray::prelude::*;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Node {
    Round,
    Square,
    Empty,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Node::Round => 'O',
            Node::Square => '#',
            Node::Empty => '.',
        })
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

type Grid = Array2<Node>;

fn parse(input: &str) -> Grid {
    let mut width = 0;
    let mut nodes = Vec::new();

    for line in input.lines() {
        nodes.extend(line.chars().map(|c| match c {
            '.' => Node::Empty,
            'O' => Node::Round,
            '#' => Node::Square,
            _ => panic!("unexpected symbol"),
        }));

        let w = line.len();
        if width == 0 {
            width = w;
        }
        else if w != width {
            panic!("unexpected line width");
        }
    }
    Grid::from_shape_vec((nodes.len() / width, width), nodes).unwrap()
}

fn compute_weight(grid: &Grid) -> usize {
    (0..grid.len_of(Axis(1))).map(|c| {
        grid.column(c).iter().rev().enumerate().map(|(w, n)| {
            if *n == Node::Round { w+1 } else { 0 }
        }
        ).sum::<usize>()
    }).sum()
}

fn tilt(grid: &mut Grid, dir: Direction) {
    let (axis, node) = match dir {
        Direction::North => (Axis(1), Node::Round),
        Direction::South => (Axis(1), Node::Empty),
        Direction::West  => (Axis(0), Node::Round),
        Direction::East  => (Axis(0), Node::Empty),
    };

    for c in 0..grid.len_of(axis) {
        let mut vs = grid.index_axis_mut(axis, c);
        let n = vs.len();

        let mut a = vs.iter().position(|x| *x != Node::Square).unwrap_or(n);
        while a < n {
            let b = vs.slice(s![a..]).iter().position(|x| *x == Node::Square).map(|x| x + a).unwrap_or(n);
            if a + 1 < b {
                let mut l = a;
                let mut r = b - 1;

                while l < r {
                    if vs[r] == node {
                        vs.swap(r, l);
                        l += 1;
                    }
                    else {
                        r -= 1;
                    }
                }
            }
            a = b + 1;
        }
    }
}

fn cycle(grid: &mut Grid) {
    for d in [Direction::North, Direction::West, Direction::South, Direction::East] {
        tilt(grid, d);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = parse(input);
    tilt(&mut map, Direction::North);
    Some(compute_weight(&map))
}

pub fn part_two(input: &str) -> Option<usize> {
    const K: usize = 1_000_000_000;
    let mut map = parse(input);

    let mut mem = std::collections::HashMap::new();
    for curr in 0..K {
        if let Some(prev) = mem.get(&map) {
            let n = curr - prev;
            let remaining = (K - curr) % n;
            for _ in 0..remaining {
                cycle(&mut map);
            }
            break;
        }
        else {
            let k = map.clone();
            cycle(&mut map);
            mem.insert(k, curr);
        }
    }

    Some(compute_weight(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
