use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Ord, Eq, PartialOrd, PartialEq)]
enum HandType {
    Highest = 0,
    Pair = 1,
    DoublePair = 2,
    ThreeOfKind = 3,
    FullHouse = 4,
    FourOfKind = 5,
    FiveOfKind = 6,
}

type CardValue = u8;

#[derive(Eq, PartialOrd, PartialEq)]
struct Hand {
    cards: [CardValue; 5],
}

trait HandTrait {
    fn hand_type(&self) -> HandType;
}
impl HandTrait for Hand {
    fn hand_type(&self) -> HandType {
        let mut cards = self.cards;
        cards.sort_unstable();

        let jokers = cards.partition_point(|x| *x == 1);
        let mut groups = cards[jokers..].into_iter().group_by(|x| *x).into_iter()
            .map(|(_, g)| g.count()).collect::<Vec<usize>>();
        groups.sort_unstable();
        groups.reverse();

        match (groups.get(0).map_or(jokers, |x| x + jokers), groups.get(1)) {
            (3, Some(2)) => HandType::FullHouse,
            (2, Some(2)) => HandType::DoublePair,
            (5, _) => HandType::FiveOfKind,
            (4, _) => HandType::FourOfKind,
            (3, _) => HandType::ThreeOfKind,
            (2, _) => HandType::Pair,
            _ => HandType::Highest,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.hand_type(), self.cards).cmp(&(other.hand_type(), other.cards))
    }
}

fn parse(input: &str, j_as_joker: bool) -> Vec<(Hand, u32)> {
    input.lines().map(|l| {
        (Hand { 
            cards: l.chars().take(5).map(|x| match x {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => if j_as_joker { 1 } else { 11 },
                        'T' => 10,
                        d => d.to_digit(10).unwrap()
                    } as u8).collect::<Vec<u8>>().try_into().unwrap()
        }, l[6..].parse::<u32>().unwrap())
    }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cards = parse(input, false);
    cards.sort_unstable_by(|(h0, _), (h1, _)| h0.cmp(h1));
    let result = cards.iter().enumerate().map(|(rank, (_, bet))| ((rank+1) as u32)*bet ).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = parse(input, true);
    cards.sort_unstable_by(|(h0, _), (h1, _)| h0.cmp(h1));
    let result = cards.iter().enumerate().map(|(rank, (_, bet))| ((rank+1) as u32)*bet ).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
