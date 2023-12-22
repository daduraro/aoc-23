use ndarray::prelude::*;

advent_of_code::solution!(22);

type Brick = [[usize; 3]; 2];

fn parse(input: &str) -> Vec<Brick> {
    input.lines().map(|line| {
        let (left, right) = line.split_once('~').unwrap();
        let mut left_it = left.split(',').map(|x| x.parse::<usize>().unwrap());
        let mut right_it = right.split(',').map(|x| x.parse::<usize>().unwrap());

        [
            [left_it.next().unwrap(), left_it.next().unwrap(), left_it.next().unwrap()],
            [right_it.next().unwrap(), right_it.next().unwrap(), right_it.next().unwrap()],
        ]
    }).collect()
}

fn settle(bricks: &mut [Brick]) -> Vec<Vec<usize>> {
    bricks.sort_by_key(|v| v[0][2] );

    let xy = bricks.iter().fold((0,0), |(x, y), b| {
        (x.max(b[1][0]), y.max(b[1][1]))
    });
    let xy = (xy.0 + 1, xy.1 + 1);

    let mut floor = ndarray::Array2::from_elem(xy, (None, 0));

    let mut result = Vec::new();
    for (idx, [[x0, y0, z0], [x1, y1, z1]]) in bricks.iter_mut().enumerate() {
        let z = *z1 - *z0 + 1;
        let (supporting, at_z) = floor.slice(s![ *x0..=*x1, *y0..=*y1 ]).iter()
            .fold((vec![], 0), |(mut s, curr_z), (which, z)|{
                if curr_z <= *z {
                    if curr_z < *z { s.clear(); }
                    if let Some(i) = which { 
                        if !s.contains(i) { s.push(*i); } }
                    (s, *z)
                }
                else {
                    (s, curr_z)
                }
            });
        
        for c in floor.slice_mut(s![ *x0..=*x1, *y0..=*y1 ]) {
            *c = (Some(idx), at_z + z);
        }

        result.push(supporting);
    }

    result
}

fn count_falling(brick: usize, supporting: &mut Vec<Vec<usize>>, supported_by: &mut Vec<Vec<usize>>) -> u32 {
    let mut result = 0;

    let mut chain = Vec::new();
    for i in std::mem::take(&mut supporting[brick]) {
        let idx = supported_by[i].iter().position(|x| *x == brick);
        supported_by[i].remove(idx.unwrap());
        if supported_by[i].is_empty() {
            chain.push(i);
        }
    }

    for brick in chain {
        result += count_falling(brick, supporting, supported_by) + 1;
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks = parse(input);
    let supported_by = settle(&mut bricks);

    let mut supporting = std::collections::HashSet::new();
    for s in supported_by {
        if s.len() == 1 {
            supporting.insert(s[0]);
        }
    }

    Some((bricks.len() - supporting.len()) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bricks = parse(input);
    let supported_by = settle(&mut bricks);

    let mut spof = std::collections::HashSet::new();
    for s in supported_by.iter() {
        if s.len() == 1 {
            spof.insert(s[0]);
        }
    }

    // revert the graph
    let mut supporting = vec![vec![]; supported_by.len()];
    for (idx, s) in supported_by.iter().enumerate() {
        for i in s {
            supporting[*i].push(idx);
        }
    }

    let mut result = 0;
    for b in spof {
        result += count_falling(b, &mut supporting.clone(), &mut supported_by.clone());
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
