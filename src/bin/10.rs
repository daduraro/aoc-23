advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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

#[derive(Debug)]
enum Node {
    Empty,
    Connection(Direction, Direction),
}

fn opposite(dir: Direction) -> Direction {
    match dir {
        Direction::N => Direction::S,
        Direction::S => Direction::N,
        Direction::W => Direction::E,
        Direction::E => Direction::W,
    }
}

#[derive(Debug)]
struct Map {
    nodes: Vec<Node>,
    width: usize,
    height: usize,
    start: usize,
}

fn from_xy(x: isize, y: isize, width: usize) -> Option<usize> {
    if x < 0 || y < 0 || (x as usize) > width {
        None
    } else {
        Some((x as usize) + (y as usize) * width)
    }
}

fn to_xy(i: usize, width: usize) -> (isize, isize) {
    ((i % width) as isize, (i / width) as isize)
}

fn advance(width: usize, height: usize, from: usize, dir: Direction) -> Option<usize> {
    let (x, y) = to_xy(from, width);
    let (dx, dy) = match dir {
        Direction::N => (0, -1),
        Direction::S => (0, 1),
        Direction::W => (-1, 0),
        Direction::E => (1, 0),
    };
    let (nx, ny) = (x + dx, y + dy);
    from_xy(nx, ny, width).and_then(|i| {
        if i < width * height {
            Some(i)
        } else {
            None
        }
    })
}

fn node_directions(node: &Node) -> Option<(Direction, Direction)> {
    match node {
        Node::Empty => None,
        Node::Connection(a, b) => Some((*a, *b)),
    }
}

fn neighbours(map: &Map, node: usize) -> Vec<usize> {
    if let Some((a, b)) = node_directions(&map.nodes[node]){
        [a, b].iter().filter_map(|dir| advance(map.width, map.height, node, *dir)).collect()
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

    for line in input.lines() {
        for c in line.chars() {
            let v = match c {
                'S' => None,
                '.' => Some(Node::Empty),
                '|' => Some(Node::Connection(Direction::N, Direction::S)),
                '-' => Some(Node::Connection(Direction::W, Direction::E)),

                'J' => Some(Node::Connection(Direction::N, Direction::W)),
                'L' => Some(Node::Connection(Direction::N, Direction::E)),

                '7' => Some(Node::Connection(Direction::S, Direction::W)),
                'F' => Some(Node::Connection(Direction::S, Direction::E)),
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

    let start = nodes.iter().position(|x| x.is_none()).unwrap();

    let conns = (0..4).map(|d| Direction::try_from(d).unwrap()).map(|dir| {
        if let Some(n) = advance(width, height, start, dir) {
            (dir, connects(nodes[n].as_ref().unwrap(), opposite(dir)))
        }
        else {
            (dir, false)
        }
    }).filter_map(|(dir, conn)| conn.then_some(dir)).collect::<Vec<_>>();

    if conns.len() != 2 {
        panic!("expected 2 connections to the start node, found {}", conns.len());
    }

    nodes[start] = Some(Node::Connection(conns[0], conns[1]));

    Map { nodes: nodes.into_iter().map(|x| x.unwrap()).collect(), width, height, start }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut current = map.start;

    let mut prev = current;
    current = neighbours(&map, current).into_iter().next().unwrap();

    let mut n = 1;
    while current != map.start {
        let next = neighbours(&map, current).into_iter().find(|x| *x != prev).unwrap();
        prev = current;
        current = next;
        n += 1;
    }

    Some(n / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);

    let is_loop = {
        let mut visited = Vec::new();
        visited.resize_with(map.nodes.len(), || false);

        let mut curr = map.start;
        let mut next = neighbours(&map, map.start).into_iter().next().unwrap();

        visited[curr] = true;
        while next != map.start {
            let prev = curr;
            curr = next;
            visited[curr] = true;
            next = neighbours(&map, curr).into_iter().find(|x| *x != prev).unwrap();
        }
        visited
    };

    let result = (0..map.nodes.len()).filter(|i| {
        if is_loop[*i] { false } // filter out the main loop
        else {
            let a = (i / map.width) * map.width;
            let parity = (a..*i).filter(|j| {
                if !is_loop[*j] { false }
                else if let Node::Connection(a, _) = map.nodes[*j] { // count half top intersection
                    a == Direction::N
                }
                else {
                    false
                }
            }).count();
            parity % 2 == 1
        }
    }).count();

    Some(result as u32)
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
