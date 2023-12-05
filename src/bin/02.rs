use regex::Regex;
use std::cmp::max;

advent_of_code::solution!(2);

#[derive(Debug, Default)]
struct CubeCount {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    rounds: Vec<CubeCount>,
}

trait Queryable {
    fn query(&self, query: &CubeCount) -> bool;
}

impl Queryable for Game {
    fn query(&self, query: &CubeCount) -> bool {
        self.rounds.iter().all(|v| {
            v.red_count <= query.red_count && v.green_count <= query.green_count && v.blue_count <= query.blue_count
        })
    }
}

fn parse(input: &str) -> Vec<Game> {
    let game_re = Regex::new(r"Game (\d+)").unwrap();
    let cube_re = Regex::new(r"(\d+) (\w+)").unwrap();

    input.lines().map(|line| {
        let mut it = line.split(':');
        let header = it.next().unwrap();
        let body = it.next().unwrap();
        if it.next().is_some() {
            panic!("Too many colons");
        }

        let game_id = game_re.captures(header).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
        let rounds = body.split(';').map(|round| {
            let cubes: CubeCount = round.split(',').map(|cube| -> (String, u32) {
                let cube = cube_re.captures(cube).unwrap();
                let count = cube.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color = cube.get(2).unwrap().as_str().to_string();
                (color, count)
            }).fold(CubeCount{ ..Default::default() }, |mut acc, (color, count)| {
                match color.as_str() {
                    "red" => acc.red_count = count,
                    "green" => acc.green_count = count,
                    "blue" => acc.blue_count = count,
                    _ => panic!("Invalid color")
                }
                acc
            });
            cubes
        }).collect::<Vec<_>>();
        Game {
            game_id,
            rounds,
        }
    }).collect()
}



pub fn part_one(input: &str) -> Option<u32> {
    let query = CubeCount {
        red_count: 12,
        green_count: 13,
        blue_count: 14,
    };
    Some(parse(input).iter().filter(|game| {
        game.query(&query)
    }).map(|g| g.game_id).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).iter().map(|game| {
        let CubeCount{ red_count, blue_count, green_count } = game.rounds.iter().fold(CubeCount{ ..Default::default() }, |mut acc, round| {
            acc.red_count = max(acc.red_count, round.red_count);
            acc.green_count = max(acc.green_count, round.green_count);
            acc.blue_count = max(acc.blue_count, round.blue_count);
            acc
        });
        red_count * green_count * blue_count
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
