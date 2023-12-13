use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Machine {
    Unknown,
    Broken,
    Operational,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Machine::Broken => "#",
            Machine::Unknown => "?",
            Machine::Operational => ".",
        })
    }
}

type Entry = (Vec<Machine>, Vec<usize>);
fn parse(input: &str) -> Vec<Entry> {
    input.lines().map(|line|{
        let s = line.split(' ').collect_vec();
        (s[0].chars().map(|s| match s {
            '.' => Machine::Operational,
            '?' => Machine::Unknown,
            '#' => Machine::Broken,
            _ => panic!("unexpected symbol"),
        }).collect(),
        s[1].split(',').map(|s| s.parse::<usize>().unwrap()).collect())
    }).collect()
}


type State = (usize, usize);
type Mem = std::collections::HashMap<State, usize>;

fn count(mem: &mut Mem, e: &Entry, i: usize, g: usize) -> usize {
    let (vs, groups) = e;
    let k = (i, g);
    if let Some(r) = mem.get(&k) {
        return *r;
    }

    let result = (||{
        // check if we have already processed all groups
        if g >= groups.len() {
            // check there are no remaining broken machine
            if i < vs.len() && vs[i..].iter().any(|v| *v == Machine::Broken) {
                return 0;
            }
            else {
                return 1;
            }
        }

        let grp = groups[g];

        // check if current group can still fit
        if vs.len() < i + grp {
            return 0;
        }

        // we need to fit group g at current position i
        let mut result = 0;
        if vs[i..(i+grp)].iter().all(|v| *v != Machine::Operational) 
            && vs.get(i+grp).map_or(true, |v| *v != Machine::Broken)
        {
            result += count(mem, e, i + grp + 1, g + 1);
        }

        // we can now advance, only if at i we do not have Broken
        if vs[i] != Machine::Broken {
            result += count(mem, e, i + 1, g);
        }

        result
    })();

    mem.insert(k, result);

    result
}

fn solve(input: &[Entry]) -> usize {
    input.iter().map(|s|{
        let mut mem = Mem::new();
        count(&mut mem, s, 0, 0)
    }).sum()
}


pub fn part_one(input: &str) -> Option<usize> {
    let values = parse(input);
    Some(solve(&values))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut values = parse(input);
    for (state, count) in values.iter_mut() {
        let start_state = state.clone();
        let start_count = count.clone();
        for _ in 0..4 {
            state.push(Machine::Unknown);
            state.append(&mut start_state.clone());
            count.append(&mut start_count.clone());
        }
    }
    Some(solve(&values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
