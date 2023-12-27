use ndarray::prelude::*;

advent_of_code::solution!(23);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Empty,
    Blocked,
    Slope(Dir),
}


fn opposite(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::S,
        Dir::S => Dir::N,
        Dir::E => Dir::W,
        Dir::W => Dir::E,
    }
}

fn clockwise(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        Dir::W => Dir::N,
    }
}

fn advance((h, w): (usize, usize), (ci, cj): (usize, usize), dir: Dir) -> Option<(usize, usize)> {
    let (di, dj) = match dir {
        Dir::N => (-1, 0),
        Dir::E => (0, 1),
        Dir::S => (1, 0),
        Dir::W => (0, -1),
    };

    let i = (ci as i32) + di;
    let j = (cj as i32) + dj;
    
    (i >= 0 && j >= 0 && i < (h as i32) && j < (w as i32)).then_some((i as usize, j as usize))
}

type Grid = Array2<Node>;

fn parse(input: &str) -> Grid {
    let mut nodes = Vec::new();
    let mut width = 0;
    for line in input.lines() {
        if width == 0 {
            width = line.len();
        }
        else if width != line.len() {
            panic!("unexpected line width");
        }

        nodes.extend(line.chars().map(|c| match c {
            '.' => Node::Empty,
            '#' => Node::Blocked,
            '>' => Node::Slope(Dir::E),
            '^' => Node::Slope(Dir::N),
            '<' => Node::Slope(Dir::W),
            'v' => Node::Slope(Dir::S),
            _ => panic!("Unexpected character"),
        }));
    }

    Array2::from_shape_vec((nodes.len() / width, width), nodes).unwrap()
}

fn count(start: (usize, usize), end: (usize, usize), dir: Dir, grid: &Grid) -> u32 {
    let (h, w) = (grid.nrows(), grid.ncols());

    let mut result = 0;
    let mut ns = [Some((start, dir)), None, None];

    while ns[1].is_none() {
        let (idx, dir) = ns[0].take().unwrap();
        if idx == end {
            return result; // found
        }
        
        let mut i = 0;

        let mut d = opposite(dir);
        for _ in 0..3 {
            d = clockwise(d);
            if let Some(neighbour) = advance((h,w), idx, d) {
                let add = match &grid[neighbour] {
                    Node::Empty => true,
                    Node::Slope(s) if *s != opposite(d) => true,
                    _ => false,
                };
                if add {
                    ns[i] = Some((neighbour, d));
                    i += 1;
                }
            }            
        }
        result += 1;
    }

    result + ns.into_iter().flatten().map(|(idx, d)| {
        count(idx, end, d, grid)
    }).max().unwrap()
}


fn count2(start: (usize, usize), end: (usize, usize), dir: Dir, grid: &Grid, mut visited: Array2<bool>) -> Option<u32> {
    let (h, w) = (grid.nrows(), grid.ncols());

    let mut result = 0;
    let mut ns = [Some((start, dir)), None, None];

    while ns[1].is_none() {
        let (idx, dir) = ns[0].take().unwrap();
        if idx == end {
            return Some(result); // found
        }

        visited[idx] = true;
        
        let mut i = 0;

        let mut d = opposite(dir);
        for _ in 0..3 {
            d = clockwise(d);
            if let Some(neighbour) = advance((h,w), idx, d) {
                let add = matches!(&grid[neighbour], Node::Empty | Node::Slope(_));
                if add && !visited[neighbour] {
                    ns[i] = Some((neighbour, d));
                    i += 1;
                }
            }
        }
        if i == 0 {
            return None; // dead-end
        }
        result += 1;
    }

    ns.into_iter().flatten().filter_map(|(idx, d)| {
        count2(idx, end, d, grid, visited.clone())
    }).max().map(|x| x + result)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    Some(count((0, 1), (grid.nrows() - 1, grid.ncols() - 2), Dir::S, &grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    count2((0, 1), (grid.nrows() - 1, grid.ncols() - 2), Dir::S, &grid, Array2::from_elem((grid.nrows(), grid.ncols()), false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
