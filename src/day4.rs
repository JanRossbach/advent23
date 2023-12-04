use regex::Regex;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
pub struct Card {
    id: usize,
    your_nums: HashSet<usize>,
    winning_nums: HashSet<usize>,
}

impl Card {
    // Takes a line of the form: "Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53"
    // The first number is the card id, the numbers after the : are the winning numbers and the numbers after the | are your numbers.
    pub fn from_line(line: &str) -> Card {
        let re = Regex::new(r"Card +(\d+): ([\d ]*)\|([ \d]*)").unwrap();
        let caps = re.captures(line).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let winning_nums = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(" ")
            .filter_map(|s| {
                s.parse::<usize>()
                    .ok()
                    .and_then(|n| if n > 0 { Some(n) } else { None })
            })
            .collect::<HashSet<usize>>();
        let your_nums = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(" ")
            .filter_map(|s| {
                s.trim()
                    .parse::<usize>()
                    .ok()
                    .and_then(|n| if n > 0 { Some(n) } else { None })
            })
            .collect::<HashSet<usize>>();
        Card {
            id,
            your_nums,
            winning_nums,
        }
    }

    pub fn num_winning(&self) -> usize {
        self.your_nums.iter().filter(|n| self.winning_nums.contains(n)).count()
    }

    pub fn points_worth(&self) -> u32 {
        let num_winning = self.num_winning();
        if num_winning == 0 {
            return 0;
        }
        let base: u32 = 2;
        base.pow(num_winning as u32 - 1)
    }
}

fn parse_input(text: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in text.lines() {
        cards.push(Card::from_line(line));
    }
    cards
}

pub fn solve(input: &str) -> u32 {
    let cards = parse_input(input);
    cards.iter().map(|c| c.points_worth()).sum()
}

pub fn solve_star2(input: &str) -> u32 {
    let cards = parse_input(input);
    let mut card_counts = HashMap::<usize,usize>::new();
    for card in cards {
        let this_card_count = *card_counts.entry(card.id).or_insert(1);
        let num_winning = card.num_winning();
        for card_id in card.id+1..=card.id+num_winning {
            card_counts.entry(card_id).and_modify(|c| *c += this_card_count).or_insert(this_card_count + 1);
        }
    }
    card_counts.values().sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_day4() {
        let input = std::fs::read_to_string("data/test_input4.txt").unwrap();
        let cards = parse_input(&input);
        assert_eq!(cards.len(), 6);
        assert_eq!(cards[0].id, 1);
        assert_eq!(cards[0].winning_nums.len(), 5);
        assert_eq!(cards[0].your_nums.len(), 8);
    }

    #[test]
    fn test_star1_test_input() {
        let input = std::fs::read_to_string("data/test_input4.txt").unwrap();
        assert_eq!(solve(&input), 13);
    }

    #[test]
    fn test_star1_real_input() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(solve(&input), 21158);
    }

    #[test]
    fn test_star2_test_input() {
        let input = std::fs::read_to_string("data/test_input4.txt").unwrap();
        assert_eq!(solve_star2(&input), 30);
    }

    #[test]
    fn test_star2_real_input() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(solve_star2(&input), 6050769);
    }
}
