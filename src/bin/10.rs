advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    N = 0,
    S = 1,
    W = 2,
    E = 3,
}

impl std::convert::TryFrom<i32> for Direction {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Direction::N as i32 => Ok(Direction::N),
            x if x == Direction::S as i32 => Ok(Direction::S),
            x if x == Direction::W as i32 => Ok(Direction::W),
            x if x == Direction::E as i32 => Ok(Direction::E),
            _ => Err(()),
        }
    }
}

fn opposite(dir: Direction) -> Direction {
    match dir {
        Direction::N => Direction::S,
        Direction::S => Direction::N,
        Direction::W => Direction::E,
        Direction::E => Direction::W,
    }
}

type Grid<T> = ndarray::Array2<T>;

#[derive(Debug, Clone, Copy)]
enum Node {
    Empty,
    Connection(Direction, Direction),
}

#[derive(Debug)]
struct Map {
    grid: Grid<Node>,
    start: (usize, usize),
}

trait MapTrait {
    fn main_loop(&self) -> MapLoop;
}

impl MapTrait for Map {
    fn main_loop(&self) -> MapLoop {
        MapLoop {
            map: self,
            curr: Some(self.start),
            next: *neighbours(self, self.start).first().unwrap(),
        }
    }
}

struct MapLoop<'a> {
    map: &'a Map,
    curr: Option<(usize, usize)>,
    next: (usize, usize),
}

impl Iterator for MapLoop<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.curr {
            let new_prev = idx;
            let new_curr = self.next;
            let new_next = neighbours(self.map, new_curr).into_iter().find(|x| *x != new_prev).unwrap();

            if new_curr == self.map.start {
                self.curr = None;
            }
            else {
                self.curr = Some(new_curr);
            }
            self.next = new_next;

            Some(new_prev)
        }
        else {
            None
        }    
    }
}

fn advance(pos: (usize, usize), size: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    let (i, j) = pos;
    let (di, dj) = match dir {
        Direction::N => (-1, 0),
        Direction::S => (1, 0),
        Direction::W => (0, -1),
        Direction::E => (0, 1),
    };
    let (ni, nj) = (i as isize + di, j as isize + dj);
    
    if ni < 0 || ni >= size.0 as isize || nj < 0 || nj >= size.1 as isize {
        None
    }
    else {
        Some((ni as usize, nj as usize))
    }
}

fn neighbours(map: &Map, node: (usize, usize)) -> Vec<(usize, usize)> {
    if let Node::Connection(a, b) = map.grid[node] {
        [a, b].into_iter().filter_map(|dir| advance(node, (map.grid.nrows(), map.grid.ncols()), dir)).collect()
    }
    else {
        vec![]
    }
}

fn connects(node: &Node, dir: Direction) -> bool {
    match node {
        Node::Empty => false,
        Node::Connection(a, b) => *a == dir || *b == dir,
    }
}

fn parse(input: &str) -> Map {
    let mut nodes = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut start = (0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let v = match c {
                'S' => {
                    start = (i, j);
                    Node::Empty
                },
                '.' => Node::Empty,
                '|' => Node::Connection(Direction::N, Direction::S),
                '-' => Node::Connection(Direction::W, Direction::E),

                'J' => Node::Connection(Direction::N, Direction::W),
                'L' => Node::Connection(Direction::N, Direction::E),

                '7' => Node::Connection(Direction::S, Direction::W),
                'F' => Node::Connection(Direction::S, Direction::E),
                _ => panic!("unexpected character"),
            };
            nodes.push(v);
        }
        if width == 0 {
            width = line.len();
        } else if width != line.len() {
            panic!("inconsistent line length");
        }
        height += 1;
    }

    let mut grid = Grid::from_shape_vec((height, width), nodes).unwrap();
    let conns = (0..4).map(|d| Direction::try_from(d).unwrap()).map(|dir| {
        if let Some(n) = advance(start, (height, width), dir) {
            let conn = connects(&grid[n], opposite(dir));
            (dir, conn)
        }
        else {
            (dir, false)
        }
    }).filter_map(|(dir, conn)| conn.then_some(dir)).collect::<Vec<_>>();

    if conns.len() != 2 {
        panic!("expected 2 connections to the start node, found {}", conns.len());
    }

    grid[start] = Node::Connection(conns[0], conns[1]);

    Map { grid, start }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    Some(map.main_loop().count() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);

    let mut is_loop = Grid::<bool>::from_elem((map.grid.nrows(), map.grid.ncols()), false);
    for idx in map.main_loop() {
        is_loop[idx] = true;
    }
    let is_loop = is_loop;

    let result = (0..map.grid.nrows()).map(|i| {
        map.grid.row(i).indexed_iter().fold((0u32, 0u32), |(mut acc, mut intersections), (j, n)|{
            if is_loop[(i, j)] {
                if let Node::Connection(a, _) = n {
                    if *a == Direction::N {
                        intersections += 1;
                    }
                }
            }
            else if intersections % 2 == 1 {
                acc += 1;
            }
            (acc, intersections)
        }).0
    }).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_c() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_part_two_d() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 6));
        assert_eq!(result, Some(10));
    }

}
