use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let first_digit = Regex::new(r"(\d)").unwrap();
    let last_digit = Regex::new(r".*(\d)").unwrap();
    Some(input.lines().map(|line| {
        let a = first_digit.captures(line.as_ref()).unwrap().get(1).unwrap().as_str();
        let b = last_digit.captures(line.as_ref()).unwrap().get(1).unwrap().as_str();
        a.parse::<u32>().and_then(|a| b.parse::<u32>().map(|b| 10*a + b))
    }).try_fold(0, |acc, x| {
        x.map(|x| acc + x)
    }).unwrap() )
}

pub fn part_two(input: &str) -> Option<u32> {
    let digit = r"(\d|one|two|three|four|five|six|seven|eight|nine)";
    let first_digit = Regex::new(digit).unwrap();
    let last_digit = Regex::new(format!(r".*{}", digit).as_str()).unwrap();
    let parse_number = |s| {
        match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            x => x.parse::<u32>().unwrap()
        }
    };
    Some(input.lines().map(|line| {
        let a = first_digit.captures(line).unwrap().get(1).unwrap();
        let b = last_digit.captures(line).unwrap().get(1).unwrap();
        10 * parse_number(a.as_str()) + parse_number(b.as_str())
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
