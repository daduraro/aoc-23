advent_of_code::solution!(8);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: usize,
    right: usize,
}

type Network = Vec<Node>;

fn parse(input: &str) -> (Vec<Direction>, Network) {
    let mut iter = input.lines();
    let dirs = iter.next().unwrap().trim().chars().map(|x| match x {
        'R' => Direction::Right,
        'L' => Direction::Left,
        _ => panic!("unexpected first line"),
    }).collect::<Vec<_>>();

    let re = regex::Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
    let net = iter.filter_map(|l| {
        re.captures(l).map(|c| c.extract().1)
    }).collect::<Vec<_>>();

    let map = net.iter().enumerate().map(|(idx, [a, _, _])| (a.to_string(), idx))
        .collect::<std::collections::HashMap<String, usize>>();

    let net = net.into_iter().map(|[name, left, right]| {
        Node {
            name: name.to_string(),
            left: *map.get(left).unwrap(),
            right: *map.get(right).unwrap(),
        }
    }).collect::<Vec<_>>();
    (dirs, net)
}


pub fn part_one(input: &str) -> Option<u32> {
    let (dirs, net) = parse(input);

    let mut curr = net.iter().position(|Node{name, ..}| name == "AAA").unwrap();
    let mut n = 0u32;
    while net[curr].name != "ZZZ" {
        curr = match dirs[(n as usize) % dirs.len()] {
            Direction::Left => net[curr].left,
            Direction::Right => net[curr].right,
        };
        n += 1;
    }
    Some(n)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (dirs, net) = parse(input);

    let curr = net.iter().enumerate().filter(|(_, Node{name, ..})| {
        name.ends_with('A')
    }).map(|(idx, _)| idx).collect::<Vec<_>>();

    // assume for now that LCM works...
    let res = curr.iter().map(|c| {
        let mut x = *c;
        let mut n = 0u64;
        while !net[x].name.ends_with('Z') {
            x = match dirs[(n as usize) % dirs.len()] {
                Direction::Left => net[x].left,
                Direction::Right => net[x].right,
            };
            n += 1;
        }
        n
    }).fold(1, num::integer::lcm);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }
}
