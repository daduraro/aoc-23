advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    _id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}


fn parse(input: &str) -> Vec<Card> {
    input.lines().map(|line| {
        let parts = line.split(':').collect::<Vec<_>>();
        let id = parts[0].strip_prefix("Card").unwrap().trim().parse::<u32>().unwrap();

        let number_parts = parts[1].split('|').collect::<Vec<_>>();

        let winning = number_parts[0].split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap() ).collect::<Vec<_>>();
        let numbers = number_parts[1].split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap() ).collect::<Vec<_>>();

        Card { _id: id, winning, numbers }
    }).collect()
}

fn winnings(cards: &[Card]) -> Vec<u32> {
    cards.iter().map(|card| {
        let mut winning = 0;
        for number in &card.winning {
            if card.numbers.contains(number) {
                winning += 1;
            }
        }
        winning
    }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse(input);
    let result = winnings(&cards).into_iter().map(|w| {
        if w > 0 { 1 << (w-1) }
        else { 0 }
    }).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse(input);
    let winning = winnings(&cards);
    let n = cards.len();
    let mut copies = std::iter::repeat(1).take(n).collect::<Vec<u32>>();
    for (idx, wins) in winning.into_iter().enumerate() {

        for i in idx+1..std::cmp::min(idx+(wins as usize)+1, n) {
            copies[i] += copies[idx];
        }
    }

    Some(copies.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
