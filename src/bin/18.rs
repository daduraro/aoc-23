use itertools::Itertools;
advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    E = 0,
    S = 1,
    W = 2,
    N = 3,
}

type Input = Vec<(Direction, isize)>;

fn parse(input: &str) -> (Input, Input) {
    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in input.lines() {
        let mut it = line.split_ascii_whitespace();
        let dir = match it.next().unwrap() {
            "R" => Direction::E,
            "L" => Direction::W,
            "U" => Direction::N,
            "D" => Direction::S,
            _ => panic!("invalid direction"),
        };
        let n = it.next().unwrap().parse::<isize>().unwrap();
        first.push((dir, n));
    
        let b: (Direction, isize) = {
            let x: &[_] = &['#', '(', ')'];
            let s = it.next().unwrap().trim_matches(x);
            let mut cs = s.chars().map(|x| x.to_digit(16).unwrap());
            let n = cs.by_ref().take(5).fold(0, |acc, i| acc*16+i) as isize;
            let dir = match cs.next().unwrap() {
                0 => Direction::E,
                1 => Direction::S,
                2 => Direction::W,
                3 => Direction::N,
                _ => panic!("unexpected value for dir"),
            };
            (dir, n)
        };
        second.push(b);
    }
    (first, second)
}

fn advance((i, j): (isize, isize), d: Direction, t: isize) -> (isize, isize) {
    match d {
        Direction::E => (i, j+t),
        Direction::N => (i-t, j),
        Direction::S => (i+t, j),
        Direction::W => (i, j-t),
    }
}

#[allow(dead_code)]
fn solve_count_parity(input: &[(Direction, isize)]) -> usize {
    let mut loop_coords = std::collections::HashSet::new();
    let mut curr = (0, 0);
    for (dir, times) in input {
        for _ in 0..*times {
            curr = advance(curr, *dir, 1);
            loop_coords.insert(curr);
        }
    }

    let min_coords = loop_coords.iter().fold((isize::MAX, isize::MAX), |acc, (i, j)|{ (acc.0.min(*i), acc.1.min(*j)) });
    let max_coords = loop_coords.iter().fold((isize::MIN, isize::MIN), |acc, (i, j)|{ (acc.0.max(*i), acc.1.max(*j)) });

    let mut result = 0;
    for i in min_coords.0..(max_coords.0+1) {
        let mut inters = 0;
        for j in min_coords.1..(max_coords.1+1) {
            if loop_coords.contains(&(i,j)) {
                result += 1;
                if loop_coords.contains(&(i-1, j)) { // only count vertical intersections towards north
                    inters += 1;
                }
            }
            else if inters % 2 == 1 { // inside
                result += 1;
            }
        }
    }
    result
}

fn sum_ranges(ranges: &[(isize, isize)]) -> usize {
    ranges.iter().map(|(a, b)|{
        assert!(b >= a);
        (b-a+1) as usize
    }).sum()
}

fn merge(inside: &mut Vec<(isize, isize)>, (a, b): &(isize, isize)) -> usize {
    assert!(a <= b);
    let n = (*b - *a + 1) as usize;
    if let Some((p, _)) =  inside.iter().find_position(|(l, _r)| l == a) {
        // we are subtracting the edge from the left
        let (_, r) = inside[p];
        if *b < r { inside[p] = (*b, r); }
        else {
            assert!(*b == r);
            inside.remove(p);
        }
        0
    }
    else if let Some((p, _)) = inside.iter().find_position(|(_l, r)| r == a) {
        // we are adding the edge to the right
        let l = inside[p].0;
        inside[p] = (l, *b);
        if p + 1 < inside.len() && inside[p+1].0 == *b { // check if we need to merge ranges
            inside[p+1].0 = l;
            inside.remove(p); // merge both ranges
            n - 2 // both endpoints were already in ranges
        }
        else {
            n - 1 // one endpoint was already in ranges
        }
    }
    else if let Some((p, _)) =  inside.iter().find_position(|(_l, r)| r == b) {
        // we are subtracting the edge from the left
        let (l, _) = inside[p];
        if l < *a { inside[p] = (l, *a); }
        else {
            assert!(l == *a);
            inside.remove(p);
        }
        0
    }
    else if let Some((p, _)) = inside.iter().find_position(|(l, _r)| l == b) {
        // we are adding the edge to the left
        let r = inside[p].1;
        inside[p] = (*a, r);
        if p > 0 && inside[p-1].1 == *a { // check if we need to merge ranges
            inside[p-1].1 = r;
            inside.remove(p); // merge both ranges
            n - 2 // both endpoints were already in ranges
        }
        else {
            n - 1 // one endpoint was already in range
        }
    }
    else if let Some((p, _)) = inside.iter().find_position(|(l, r)| l < a && a < r) {
        // we need to partition current range
        let (l, r) = inside[p];
        assert!(l < *a && l < *b && *a < r && *b < r);
        inside[p].1 = *a;
        inside.insert(p+1, (*b, r));
        0
    }
    else {
        // unrelated edge: add it
        inside.insert(inside.partition_point(|(l, _)| l < a), (*a, *b));
        n
    }
}

#[allow(dead_code)]
fn solve_sweepline(input: &[(Direction, isize)]) -> usize {
    let mut edges = Vec::new();
    let mut curr = (0,0);
    for (dir, t) in input {
        let a = curr;
        let b = advance(curr, *dir, *t);
        
        // insert only horizontal edges
        if a.0 == b.0 { 
            if a.1 < b.1 { edges.push((a.0, a.1, b.1)); }
            else { edges.push((a.0, b.1, a.1)); }
        }

        curr = b;
    }

    edges.sort();


    let mut inside = Vec::new();
    let mut i_prev = isize::MIN;
    let mut result = 0;

    for (i, a, b) in edges {
        assert!(a <= b);
        if i > i_prev && !inside.is_empty() {
            result += sum_ranges(&inside) * (i - i_prev) as usize;
        }
        result += merge(&mut inside, &(a, b));
        i_prev = i;
        assert!(inside.windows(2).all(|w| w[0] <= w[1]));
        assert!(inside.windows(2).all(|w| w[0].1 < w[1].0));
    }
    assert!(inside.is_empty());

    result
}

#[allow(dead_code)]
fn solve_area(input: &[(Direction, isize)]) -> usize {
    // Imagine the polygon at integer coordinates:
    // +--+
    // |..|
    // |..|
    // +--+
    // this has an area of 3x3=9, 4 interior nodes and 12 boundary nodes.
    // The area (a) can be easily computed by summing up
    // the area of the triangles wrt origin of coordinates,
    // and the interior points with Pick's theorem of
    //  a = interior + boundary/2 - 1 => interior = a - boundary/2 + 1
    // so that interior = 9 - 12/2 + 1 = 4.
    // If we "inflate" each node so that it occupies a whole square,
    // then the total area (A) is interior + boundary, and so it follows
    // that A = interior + boundary 
    //        = (a - boundary/2 + 1) + boundary 
    //        =  a + boundary/2 + 1
    let mut boundary = 0;
    let mut area = 0;
    let mut prev = (0,0);
    for (d, t) in input {
        let curr = advance(prev, *d, *t);
        boundary += *t as usize;
        area += prev.0*curr.1 - curr.0*prev.1;
        prev = curr;
    }
    area /= 2;
    boundary / 2 + area.unsigned_abs() + 1
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(solve_area(&input.0))
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(solve_area(&input.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
