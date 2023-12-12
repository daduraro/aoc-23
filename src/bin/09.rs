advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn differences(values: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(values.len() - 1);
    for i in 0..values.len() - 1 {
        result.push(values[i + 1] - values[i]);
    }
    result
}

pub fn part_one(input: &str) -> Option<i64> {
    let values = parse(input);

    let result = values.into_iter().map(|seq| {
        let mut diffs = Vec::new();
        diffs.push(seq);
        while !diffs.last().unwrap().iter().all(|x| *x == 0) {
            let last = diffs.last().unwrap();
            let diff = differences(last);
            diffs.push(diff);
        }
        diffs.into_iter().rev().skip(1).fold(0, |acc, x| {
            acc + x.last().unwrap()
        })
    }).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let values = parse(input);

    let result = values.into_iter().map(|seq| {
        let mut diffs = Vec::new();
        diffs.push(seq);
        while !diffs.last().unwrap().iter().all(|x| *x == 0) {
            let last = diffs.last().unwrap();
            let diff = differences(last);
            diffs.push(diff);
        }
        diffs.into_iter().rev().skip(1).fold(0, |acc, x| {
            x.first().unwrap() - acc
        })
    }).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
