advent_of_code::solution!(5);

type Range = (usize, usize);

#[derive(Debug)]
struct MapRange {
    src: Range,
    dst: usize,
}

trait Converter {
    fn convert(&self, x: usize) -> Option<usize>;
    fn convert_range(&self, r: &Range) ->(Vec<Range>, Option<Range>);
}

impl Converter for MapRange {
    fn convert(&self, x: usize) -> Option<usize> {
        if x < self.src.0 || x >= self.src.0 + self.src.1 {
            None
        }
        else {
            Some(self.dst + x - self.src.0)
        }
    }
    fn convert_range(&self, r: &Range) -> (Vec<Range>, Option<Range>) {
        let src_a = self.src.0;
        let src_b = self.src.0 + self.src.1;

        let r_a = r.0;
        let r_b = r.0 + r.1;

        if src_a >= r_b || r_a >= src_b {
            (vec![*r], None)
        }
        else {
            let mut unchanged = Vec::new();
            if r_a < src_a {
                unchanged.push((r_a, src_a - r_a));
            }
            if r_b > src_b {
                unchanged.push((src_b, r_b - src_b));
            }
            let a = std::cmp::max(r_a, src_a);
            let b = std::cmp::min(r_b, src_b);
            (unchanged, Some((a - src_a + self.dst, b - a)))
        }
    }
}

#[derive(Debug)]
struct Map {
    _from: String,
    _to: String,
    ranges: Vec<MapRange>,
}

type Input = (Vec<usize>, Vec<Map>);
fn parse(input: &str) -> Input {
    let lines = input.lines().collect::<Vec<_>>();

    let (seeds, body) = lines.split_first().unwrap();

    let seeds = seeds.strip_prefix("seeds: ").unwrap().split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let parse_header = |s: &str| {
        s.strip_suffix(" map:").map(|h| {
            let parts = h.split("-to-").collect::<Vec<_>>();
            (parts[0].to_string(), parts[1].to_string())
        })
    };

    let parse_mapping = |s: &str| {
        let parts = s.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        if parts.len() == 3 {
            Some(MapRange{ dst: parts[0], src: (parts[1], parts[2]) })
        }
        else {
            None
        }
    };

    let mut maps = Vec::new();
    let mut map = None;
    for line in body {
        if let Some((_from, _to)) = parse_header(line) {
            if let Some(m) = map.replace(Map{ _from, _to, ranges: vec![]}) {
                maps.push(m);
            }
        }
        else if let Some(range) = parse_mapping(line) {
            if let Some(m) = &mut map {
                m.ranges.push(range);
            }
            else {
                panic!("no header found");
            }
        }
    }
    if let Some(m) = map.take() {
        maps.push(m);
    }

    (seeds, maps)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (seeds, maps) = parse(input);
    seeds.iter().map(|s| {
        maps.iter().fold(*s, |s, Map{ ranges, ..}| {
            ranges.iter().find_map(|r| r.convert(s)).unwrap_or(s)
        })
    }).min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (seeds, maps) = parse(input);
    let seed_ranges = seeds.chunks(2).map(|seed_range| (seed_range[0], seed_range[1])).collect::<Vec<_>>();
    maps.iter().fold(seed_ranges, |seeds, Map{ ranges, ..}| {
        let mut processed = Vec::<Range>::new();
        let mut unprocessed = ranges.iter().fold(seeds, |unprocessed, range| {
            let mut to_be_processed = Vec::<Range>::new();
            for x in unprocessed {
                let (mut u, p) = range.convert_range(&x);
                if let Some(converted) = p {
                    processed.push(converted);
                }
                to_be_processed.append(&mut u)
            }
            to_be_processed
        });
        processed.append(&mut unprocessed); // identity for all those values that no range include
        processed
    }).iter().map(|(a,_)| *a).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
