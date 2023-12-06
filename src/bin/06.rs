advent_of_code::solution!(6);

fn parse(input: &str) -> Vec<(u32, u32)> {
    let lines = input.lines().collect::<Vec<_>>();
    std::iter::zip(
        lines[0].strip_prefix("Time:").unwrap().trim().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()),
        lines[1].strip_prefix("Distance:").unwrap().trim().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap())
    ).collect()
}

fn find_solution(t: u64, d: u64) -> u64 {
    // a + b = t
    // a * b > d
    // (t +- sqrt(t² - 4d)) / 2

    let s = ((t*t - 4 * d) as f64).sqrt();

    let a = (((t as f64 - s) / 2.)).floor() as u64 + 1;
    let b = (((t as f64 + s) / 2.)).ceil() as u64 - 1;
    // println!("t {:?}; d {:?}; a {:?}; b {:?}; ways {:?}", t, d, a, b, b-a+1);
    b - a + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = parse(input).into_iter().map(|(time, dist)|{
        find_solution(time as u64, dist as u64)
    }).product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (time, dist) = parse(input).into_iter().fold((0,0), |(acc_t,acc_d), (t,d)|{
        let t = t as u64;
        let d = d as u64;
        (acc_t * 10u64.pow(t.ilog10()+1) + t, acc_d * 10u64.pow(d.ilog10()+1) + d)
    });
    Some(find_solution(time, dist))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
