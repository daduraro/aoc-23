use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Node {
    Empty,
    Galaxy,
}

type Map = Vec<Vec<Node>>;

fn parse(input: &str) -> Map {
    let mut nodes = Vec::new();
    let mut width = 0;
    for line in input.lines() {
        let row = line.chars().map(|c| match c {
            '.' => Node::Empty,
            '#' => Node::Galaxy,
            _ => panic!("unexpected symbol"),
        }).collect::<Vec<_>>();
        if width == 0 {
            width = row.len();
        }
        else if width != row.len() {
            panic!("unexpected row length");
        }
        nodes.push(row);
    }
    nodes
}

fn solve(input: &str, k: usize) -> Option<usize> {
    let map = parse(input);
    let empty_rows = (0..map.len()).filter(|r| map[*r].iter().all(|n| *n == Node::Empty ) ).collect::<Vec<_>>();
    let empty_cols = (0..map.first().unwrap().len()).filter(|c| map.iter().map(|row| &row[*c]).all(|n| *n == Node::Empty) ).collect::<Vec<_>>();

    let galaxies = map.iter().enumerate().flat_map(|(r, row)| {
        row.iter().enumerate().filter(|(_, n)| **n == Node::Galaxy).map(move |(c,_)| (r, c))
    }).collect::<Vec<_>>();

    let result = galaxies.iter().combinations(2).map(|group| {
        let (r0, c0) = &group[0];
        let (r1, c1) = &group[1];

        let (rmin, rmax) = (*r0.min(r1), *r0.max(r1));
        let (cmin, cmax) = (*c0.min(c1), *c0.max(c1));       

        let rs = empty_rows.partition_point(|r| *r < rmax) - empty_rows.partition_point(|r| *r <= rmin);
        let cs = empty_cols.partition_point(|c| *c < cmax) - empty_cols.partition_point(|c| *c <= cmin);

        r0.abs_diff(*r1) + c0.abs_diff(*c1) + (rs+cs) * (k-1)
    }).sum();
    Some(result)
}


pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two_a() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(1030));
    }
    #[test]
    fn test_part_two_b() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(8410));
    }
}
