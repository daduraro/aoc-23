use std::{fmt::Write, collections::HashSet};

advent_of_code::solution!(16);

#[derive(PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
enum Node {
    Empty,
    SplitV,
    SplitH,
    SW_NE,
    SE_NW,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Node::Empty => '.',
            Node::SplitV => '|',
            Node::SplitH => '-',
            Node::SW_NE => '/',
            Node::SE_NW => '\\',
        })
    }
}

type Grid = ndarray::Array2<Node>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

fn parse(input: &str) -> Grid {
    let mut width = 0;
    let mut nodes = Vec::new();
    for line in input.lines() {
        nodes.extend(line.chars().map(|c| match c {
            '.' => Node::Empty,
            '|' => Node::SplitV,
            '-' => Node::SplitH,
            '\\' => Node::SE_NW, // E->S; S->E; N->W; W->N 
            '/' => Node::SW_NE,
            _ => panic!("unexpected character {:?}", c),
        }));
        if width == 0 {
            width = line.len();
        }
        else if width != line.len() {
            panic!("inconsistent widths")
        }
    }

    Grid::from_shape_vec((nodes.len() / width, width), nodes).unwrap()
}

fn advance((i, j): (isize, isize), direction: Direction) -> (isize, isize) {
    match direction {
        Direction::East => (i, j+1),
        Direction::North => (i-1, j),
        Direction::South => (i+1, j),
        Direction::West => (i, j-1),
    }
}

fn is_vertical(direction: Direction) -> bool {
    matches!(direction, Direction::North | Direction::South)
}

type EnergyGrid = ndarray::Array2<bool>;
fn energize_rec(grid: &Grid, energized: &mut EnergyGrid, (i, j): (isize, isize), direction: Direction, mem: &mut HashSet<(isize, isize, Direction)>) {
    if i < 0 || i >= grid.nrows() as isize || j < 0 || j >= grid.ncols() as isize { return; }
    if mem.contains(&(i,j,direction)) { return; }
    mem.insert((i,j, direction));
    let c = (i as usize, j as usize);
    energized[c] = true;

    let mut adv = |d: Direction| energize_rec(grid, energized, advance((i,j), d), d, mem);

    match (grid[c], direction) {
        (Node::Empty, d) => adv(d), 
        (Node::SplitV, d) if is_vertical(d) => adv(d),
        (Node::SplitH, d) if !is_vertical(d) => adv(d),
        (Node::SplitV, _) => {
            adv(Direction::North);
            adv(Direction::South);
        },
        (Node::SplitH, _) => {
            adv(Direction::East);
            adv(Direction::West);
        },
        (Node::SW_NE, Direction::South) => adv(Direction::West),
        (Node::SW_NE, Direction::West) => adv(Direction::South),
        (Node::SW_NE, Direction::North) => adv(Direction::East),
        (Node::SW_NE, Direction::East) => adv(Direction::North),
        (Node::SE_NW, Direction::South) => adv(Direction::East),
        (Node::SE_NW, Direction::East) => adv(Direction::South),
        (Node::SE_NW, Direction::North) => adv(Direction::West),
        (Node::SE_NW, Direction::West) => adv(Direction::North),

    }
}
fn energize(grid: &Grid, start: (isize, isize), direction: Direction) -> EnergyGrid {
    let mut result = EnergyGrid::from_elem((grid.nrows(), grid.ncols()), false);
    energize_rec(grid, &mut result, start, direction, &mut HashSet::new());
    result
}

fn count_energize(grid: &Grid, start: (isize, isize), direction: Direction) -> u32 {
    energize(grid, start, direction).into_iter().map(u32::from).sum::<u32>()
}


pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    Some(count_energize(&grid, (0,0), Direction::East))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let mut result = 0;
    for i in 0..grid.nrows() {
        result = result.max(count_energize(&grid, (i as isize, 0), Direction::East));
        result = result.max(count_energize(&grid, (i as isize, (grid.ncols() - 1) as isize), Direction::West));
    }
    for j in 0..grid.ncols() {
        result = result.max(count_energize(&grid, (0, j as isize), Direction::South));
        result = result.max(count_energize(&grid, ((grid.nrows() - 1) as isize, j as isize), Direction::North));
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
