use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
enum ModuleState {
    Flipflop(bool),
    Conjunction(Vec<bool>),
}

#[derive(Debug)]
struct Connection {
    name: String,
    output: Vec<Result<usize, String>>,
    input: Vec<usize>,
}

#[derive(Debug)]
struct Modules {
    broadcaster: usize,
    rx_in: Option<usize>,
    connections: Vec<Connection>,
    state: Vec<Option<ModuleState>>,
}

fn parse(input: &str) -> Modules {
    let mut name_map = HashMap::new();

    let mut output = Vec::new();
    let mut state = Vec::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        
        let (s, left) = match left.chars().next().unwrap() {
            '%' => (Some(ModuleState::Flipflop(false)), &left[1..]),
            '&' => (Some(ModuleState::Conjunction(Vec::new())), &left[1..]),
            _ => (None, left),
        };

        name_map.insert(left.to_string(), name_map.len());

        output.push(Vec::from_iter(right.split(", ").map(String::from)));

        state.push(s);
    }

    let output = output.into_iter().map(|ns| {
        ns.into_iter().map(|v| name_map.get(&v).cloned().ok_or(v)).collect_vec()
    }).collect_vec();

    let mut input = vec![Vec::new(); output.len()];
    for (i, out) in output.iter().enumerate() {
        for j in out.iter().flatten() {
            let v = &mut input[*j];

            let p = v.partition_point(|&x| x < i);
            v.insert(p, i);
            assert!(v.windows(2).all(|a| a[0] <= a[1] ));
        }
    }

    let mut connections = input.into_iter().zip(output).map(|(input, output)|{
        Connection { name: "".to_string(), output, input }
    }).collect_vec();

    for (i, s) in state.iter_mut().enumerate() {
        if let Some(ModuleState::Conjunction(vs)) = s.as_mut() {
            *vs = vec![false; connections[i].input.len()];
        }
    }

    let broadcaster = *name_map.get("broadcaster").unwrap();

    for (name, idx) in name_map {
        connections[idx].name = name;
    }

    let rx_in = connections.iter().position(|x| x.output.iter().any(|o| o.as_ref().is_err_and(|o| o == "rx")) );

    Modules { rx_in, broadcaster, connections, state }
}

impl Modules {
    fn signal(&mut self) -> (usize, usize) {
        let mut signals = VecDeque::new();
        let mut low_count = 1; // from aptly to broadcaster
        let mut high_count = 0;

        for out in &self.connections[self.broadcaster].output {
            signals.push_back((self.broadcaster, out.clone(), false));
        }

        while let Some((from, to, is_high)) = signals.pop_front() {
            if is_high { high_count += 1; } else { low_count += 1; }

            if let Ok(to) = to {
                if let Some(state) = &mut self.state[to] {
                    let send_signal = match state {
                        ModuleState::Flipflop(v) => {
                            if !is_high {
                                *v = !*v; // flip state
                                Some(*v)
                            }
                            else { None }
                        },
                        ModuleState::Conjunction(vs) => {
                            let i = self.connections[to].input.binary_search(&from).unwrap();
                            vs[i] = is_high;
                            let all_high = is_high && vs.iter().all(|v| *v);
                            Some(!all_high)
                        },
                    };
                    if let Some(h) = send_signal {
                        for out in &self.connections[to].output {
                            signals.push_back((to, out.clone(), h));
                        }
                    }
                }
            }
        }

        (low_count, high_count)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut modules = parse(input);

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let (incr_low, incr_high) = modules.signal();
        low_count += incr_low;
        high_count += incr_high;
    }

    Some(low_count * high_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut modules = parse(input);

    assert!(modules.rx_in.is_some());
    let rx = modules.rx_in.unwrap();

    let mut freq = vec![vec![0usize]; modules.connections[rx].input.len()];
    for pulses in 1.. {
        let mut signals = VecDeque::new();
        for out in &modules.connections[modules.broadcaster].output {
            signals.push_back((modules.broadcaster, out.clone(), false));
        }

        while let Some((from, to, is_high)) = signals.pop_front() {
            if let Ok(to) = to {
                if let Some(state) = &mut modules.state[to] {
                    let send_signal = match state {
                        ModuleState::Flipflop(v) => {
                            if !is_high {
                                *v = !*v; // flip state
                                Some(*v)
                            }
                            else { None }
                        },
                        ModuleState::Conjunction(vs) => {
                            let i = modules.connections[to].input.binary_search(&from).unwrap();
                            vs[i] = is_high;
                            let all_high = is_high && vs.iter().all(|v| *v);
                            Some(!all_high)
                        },
                    };
                    if let Some(h) = send_signal {
                        for out in &modules.connections[to].output {
                            if to == rx {
                                if let Some(ModuleState::Conjunction(vs)) = &modules.state[to] {
                                    for (i, b) in vs.iter().enumerate() {
                                        if *b && freq[i].last().cloned() != Some(pulses) {
                                            freq[i].push(pulses);
                                        }
                                    }
                                }
                            }
                            signals.push_back((to, out.clone(), h));
                        }
                    }
                }
            }
        }

        if freq.iter().all(|x| x.len() > 1) {
            let result = freq.iter().map(|x| x[x.len() - 1] - x[x.len() - 2]).fold(1, num::integer::lcm);
            return Some(result);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11687500));
    }
}
