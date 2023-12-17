advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    N,
    W,
    S,
    E,
}

fn opposite(dir: Direction) -> Direction {
    match dir {
        Direction::E => Direction::W,
        Direction::N => Direction::S,
        Direction::S => Direction::N,
        Direction::W => Direction::E,
    }
}

fn advance((i, j): (usize, usize), (h, w): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    let (di, dj) = match direction {
        Direction::E => ( 0,  1),
        Direction::N => (-1,  0),
        Direction::S => ( 1,  0),
        Direction::W => ( 0, -1),
    };
    
    let (ni, nj) = (i as isize + di, j as isize + dj);
    if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
        None
    }
    else {
        Some((ni as usize, nj as usize))
    }
}

fn parse(input: &str) -> ndarray::Array2<u8> {
    let mut width = 0;
    let mut nodes = Vec::new();
    for line in input.lines() {
        if width == 0 {
            width = line.len();
        }
        else if width != line.len() {
            panic!("inconsistent line width");
        }

        nodes.extend(line.chars().map(|x| x.to_digit(10).unwrap() as u8))
    }
    ndarray::Array2::from_shape_vec((nodes.len() / width, width), nodes).unwrap()
}

fn solve(input: &str, min_to_turn: usize, max_to_turn: usize) -> u32 {
    assert!(min_to_turn <= max_to_turn);
    type Node = ((usize, usize), (Direction, usize));
    type NodeMap = std::collections::HashMap<Node, usize>;

    let map = parse(input);
    let (h, w) = (map.nrows(), map.ncols());
    let mut gscore = NodeMap::new();

    let start: Node = ((0,0), (Direction::E, 0));
    let end = (h - 1, w - 1);

    gscore.insert(start, 0);

    let fscore = |node: &Node, score: &NodeMap| {
        let ((i, j), _) = *node;
        -((score[node] + ((end.0 - i) + (end.1 - j))) as i32)
    };
    let mut heap = std::collections::BinaryHeap::new();
    heap.push((fscore(&start, &gscore), start));

    while let Some((_, node)) = heap.pop() {
        let (idx, (dir, times)) = &node;
        if idx == &end && *times >= min_to_turn {
            return gscore[&node] as u32;
        }
        for d in [Direction::N, Direction::E, Direction::S, Direction::W] {
            if d == opposite(*dir) { 
                continue; // cannot go backwards
            }
            if d == *dir && *times >= max_to_turn {
                continue; // cannot go further that way: we reached MAX_TO_TURN
            }
            if d != *dir && *times > 0 && *times < min_to_turn  {
                continue; // cannot turn: we need to reach MIN_TO_TURN
            }
            if let Some(n_idx) = advance(*idx, (h,w), d) {
                let n_dirs = {
                    if d == *dir { (d, *times + 1) }
                    else { (d, 1) }
                };
                let tentative_g = gscore[&node] + map[n_idx] as usize;
                let neighbour = (n_idx, n_dirs);
                if tentative_g < *gscore.get(&neighbour).unwrap_or(&usize::MAX) {
                    gscore.insert(neighbour, tentative_g);
                    heap.push((fscore(&neighbour, &gscore), neighbour));
                }
            }
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input,0,3))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input,4,10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
