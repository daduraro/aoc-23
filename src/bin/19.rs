use std::{collections::HashMap, ops::RangeBounds};

use nom::{IResult, branch::alt, multi::{separated_list1, many1}, character::complete::{char, alpha1, digit1, line_ending}, Parser, sequence::{tuple, delimited, separated_pair, preceded, terminated}, bytes::complete::tag};
use ranges::{Ranges, GenericRange};
advent_of_code::solution!(19);

#[derive(Debug, Clone, Copy)]
enum PartType {
    X,
    M,
    A,
    S,
}

fn parse_part_type(input: &str) -> IResult<&str, PartType> {
    alt((
        char('x').map(|_| PartType::X),
        char('m').map(|_| PartType::M),
        char('a').map(|_| PartType::A),
        char('s').map(|_| PartType::S),
    ))(input)
}

#[derive(Debug, Clone, Copy)]
enum Comparison {
    LessThan,
    GreaterThan,
}

fn parse_comparison(input: &str) -> IResult<&str, Comparison> {
    alt((
        char('<').map(|_| Comparison::LessThan),
        char('>').map(|_| Comparison::GreaterThan),
    ))(input)
}

#[derive(Debug, Clone)]
enum Goto {
    Accept,
    Reject,
    Label(String),
}

fn parse_goto(input: &str) -> IResult<&str, Goto> {
    alt((
        char('A').map(|_| Goto::Accept),
        char('R').map(|_| Goto::Reject),
        alpha1.map(|t: &str| Goto::Label(t.to_string())),
    ))(input)
}


#[derive(Debug, Clone, Copy)]
struct Condition {
    part: PartType,
    comparison: Comparison,
    num: u32,
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    (tuple((
        parse_part_type,
        parse_comparison,
        digit1.map(|v: &str| v.parse::<u32>().unwrap())
    ))
    .map(|(part, comparison, num)| Condition {part, comparison, num}))
    .parse(input)    
}

type Process = (Option<Condition>, Goto);

#[derive(Debug, Clone)]
struct Workflow {
    label: String,
    process: Vec<Process>,
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    tuple((
        alpha1.map(|s: &str| s.to_string()),
        delimited(char('{'), separated_list1(
            char(','),
            alt((
                separated_pair(parse_condition, char(':'), parse_goto).map(|(cond, goto)| (Some(cond), goto)),
                parse_goto.map(|goto| (None, goto)),
            ))
        ), char('}'))
    )).map(|(label, process)| Workflow{ label, process })
    .parse(input)
}


struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    delimited(char('{'), tuple((
        delimited(tag("x="), digit1.map(|s: &str| s.parse::<u32>().unwrap()), char(',')),
        delimited(tag("m="), digit1.map(|s: &str| s.parse::<u32>().unwrap()), char(',')),
        delimited(tag("a="), digit1.map(|s: &str| s.parse::<u32>().unwrap()), char(',')),
        preceded(tag("s="), digit1.map(|s: &str| s.parse::<u32>().unwrap()))
    )), char('}'))
    .map(|(x, m, a, s)| Part {x, m, a, s})
    .parse(input)
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    separated_pair(
        many1(terminated(parse_workflow, line_ending)),
        line_ending,
        many1(terminated(parse_part, line_ending))
    ).parse(input).unwrap().1
}


fn check(cond: &Condition, part: &Part) -> bool {
    let v = match cond.part {
        PartType::X => part.x,
        PartType::M => part.m,
        PartType::A => part.a,
        PartType::S => part.s,
    };
    match cond.comparison {
        Comparison::GreaterThan => v > cond.num,
        Comparison::LessThan => v < cond.num,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ws, parts) = parse(input);
    let start = "in".to_string();
    let mut map = HashMap::new();
    for w in ws {
        map.insert(w.label, w.process);
    }

    let result = parts.into_iter().filter(|p|{
        let mut curr = start.clone();
        loop {
            for (cond, goto) in map.get(&curr).unwrap().iter() {
                if cond.as_ref().map(|c| check(c, p)).unwrap_or(true) {
                    match goto {
                        Goto::Accept => return true,
                        Goto::Reject => return false,
                        Goto::Label(d) => {
                            curr = d.clone();
                            break; // breaks current workflow
                        }
                    }
                }
            }
        }
    }).map(|p| p.a+p.m+p.s+p.x).sum();

    Some(result)
}

#[derive(Clone, Debug)]
struct PartRange {
    x: Ranges<u32>,
    m: Ranges<u32>,
    a: Ranges<u32>,
    s: Ranges<u32>,
}

fn range_length(r: &GenericRange<u32>) -> usize {
    let b = match r.start_bound() {
        std::ops::Bound::Unbounded => 1,
        std::ops::Bound::Included(x) => *x,
        std::ops::Bound::Excluded(x) => *x + 1,
    };

    let e = match r.end_bound() {
        std::ops::Bound::Unbounded => 4000,
        std::ops::Bound::Included(x) => *x + 1,
        std::ops::Bound::Excluded(x) => *x,
    };
    (e - b) as usize
}

fn ranges_length(r: &Ranges<u32>) -> usize {
    r.as_slice().iter().map(range_length).sum()
}

impl PartRange {
    fn all() -> Self {
        PartRange { x: Ranges::from(1..=4000), m: Ranges::from(1..=4000), a: Ranges::from(1..=4000), s: Ranges::from(1..=4000) }
    }

    fn split(value: &Condition) -> (Self, Self) {
        let range = match value.comparison {
            Comparison::GreaterThan => Ranges::from((value.num+1)..),
            Comparison::LessThan => Ranges::from(..value.num),
        };
        match value.part {
            PartType::X => (PartRange { x: range.clone(), ..PartRange::all() }, PartRange { x: range.invert(), ..PartRange::all() }),
            PartType::M => (PartRange { m: range.clone(), ..PartRange::all() }, PartRange { m: range.invert(), ..PartRange::all() }),
            PartType::A => (PartRange { a: range.clone(), ..PartRange::all() }, PartRange { a: range.invert(), ..PartRange::all() }),
            PartType::S => (PartRange { s: range.clone(), ..PartRange::all() }, PartRange { s: range.invert(), ..PartRange::all() }),
        }
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }

    fn intersect(self, other: PartRange) -> Self {
        PartRange {
            x: self.x & other.x,
            m: self.m & other.m,
            a: self.a & other.a,
            s: self.s & other.s,
        }
    }
    fn size(&self) -> usize {
        ranges_length(&self.x) * ranges_length(&self.m) * ranges_length(&self.a) * ranges_length(&self.s)
    }
}

impl std::fmt::Display for PartRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x: {}; m: {}; a: {}; s: {}", self.x, self.m, self.a, self.s))
    }
}

fn part_range(label: &str, map: &HashMap<String, Vec<Process>>, mem: &mut HashMap<String, Vec<PartRange>>) -> Vec<PartRange> {
    if let Some(range) = mem.get(label) {
        return range.clone()
    }

    let result: Vec<PartRange> = {
        let mut result = Vec::new();
        let mut mask = PartRange::all();

        for (cond, goto) in map.get(label).unwrap() {
            if mask.is_empty() {
                break; // no need to iterate more
            }

            let mut term = || match goto {
                Goto::Accept => vec![PartRange::all()],
                Goto::Reject => vec![],
                Goto::Label(lbl) => part_range(lbl, map, mem),
            };

            if let Some(cond) = cond {
                let (cond, inv) = PartRange::split(cond);

                let cond = cond.intersect(mask.clone());
                if !cond.is_empty() {
                    result.extend(term().into_iter().map(|x| x.intersect(cond.clone())).filter(|p| !p.is_empty()));
                }

                mask = mask.intersect(inv);
            }
            else {
                result.extend(term().into_iter().map(|x| x.intersect(mask.clone())).filter(|p| !p.is_empty()));
                break; // no need to iterate more
            }
        }

        result
    };

    mem.insert(label.to_string(), result.clone());
    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ws, _) = parse(input);
    let mut map = HashMap::new();
    for w in ws {
        map.insert(w.label, w.process);
    }
    let result = part_range("in", &map, &mut HashMap::new()).into_iter()
        // .map(|r| { println!("{}",r); r })
        .map(|r| r.size()).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
