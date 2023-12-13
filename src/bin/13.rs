use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Node {
    Ash,
    Rock,
}

type Map = Vec<Vec<Node>>;

fn parse(input: &str) -> Vec<Map> {
    input.lines().collect_vec().split(|s| s.is_empty()).map(|lines|{
        lines.iter().map(|row| {
            row.chars().map(|c| match c {
                '.' => Node::Ash,
                '#' => Node::Rock,
                _ => panic!("unexpected symbol"),
            }).collect()
        }).collect()
    }).collect()
}

fn find_mirror(m: &Map) -> Option<usize> {
    (1..m.len()).find(|r| 
        m[0..*r].iter().rev().zip(m[*r..m.len()].iter())
        .all(|(a, b)| a == b)
    )
}

fn find_mirror_smudge(m: &Map) -> Option<usize> {
    (1..m.len()).find(|r| 
        m[0..*r].iter().rev().zip(m[*r..m.len()].iter())
        .map(|(a, b)| {
            a.iter().zip(b).map(|(a, b)| if a == b { 0 } else { 1 }).sum::<u32>()
        }).sum::<u32>() == 1
    )
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let r = input.into_iter().map(|m|{
        find_mirror(&m).map(|x| x*100)
        .or_else(|| { find_mirror(&transpose(m)) })
        .unwrap()
    }).sum::<usize>();
    Some(r)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let r = input.into_iter().map(|m|{
        find_mirror_smudge(&m).map(|x| x*100)
        .or_else(|| { find_mirror_smudge(&transpose(m)) })
        .unwrap()
    }).sum::<usize>();
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
