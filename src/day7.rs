use core::fmt;
use std::{cmp::Ordering, fmt::Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Card {
    pub rank: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    pub fn from_char(c: char) -> Self {
        let rank = match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card rank"),
        };
        Self { rank }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    pub cards: Vec<Card>,
    pub bid: usize,
    pub hand_type: HandType,
}

impl Hand {
    pub fn from_line(line: &str) -> Self {
        let re = regex::Regex::new(r"([2-9TJQKA]{5}) (\d+)").unwrap();
        let captures = re.captures(line).unwrap();
        let cards = captures[1]
            .chars()
            .map(|c| Card::from_char(c))
            .collect::<Vec<_>>();
        let bid = captures[2].parse::<usize>().unwrap();
        let hand_type: HandType = find_type(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

fn find_type(cards: &Vec<Card>) -> HandType {
    let mut ranks = [0; 15];
    for card in cards.iter() {
        ranks[card.rank as usize] += 1;
    }
    let mut num_pairs = 0;
    let mut num_triples = 0;
    let mut num_quads = 0;
    let mut num_quints = 0;
    for count in ranks.iter() {
        if *count == 2 {
            num_pairs += 1;
        } else if *count == 3 {
            num_triples += 1;
        } else if *count == 4 {
            num_quads += 1;
        } else if *count == 5 {
            num_quints += 1;
        }
    }
    if num_quints == 1 {
        return HandType::FiveOfAKind;
    } else if num_quads == 1 {
        return HandType::FourOfAKind;
    } else if num_triples == 1 && num_pairs == 1 {
        return HandType::FullHouse;
    } else if num_triples == 1 {
        return HandType::ThreeOfAKind;
    } else if num_pairs == 2 {
        return HandType::TwoPair;
    } else if num_pairs == 1 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cards == other.cards;
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return Some(self.hand_type.cmp(&other.hand_type));
        }
        for i in 0..5 {
            if self.cards[i] == other.cards[i] {
                continue;
            } else {
                return Some(self.cards[i].cmp(&other.cards[i]));
            }
        }
        Some(Ordering::Equal)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let repr = self
            .cards
            .iter()
            .map(|c| match c.rank {
                10 => "T".to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                _ => c.rank.to_string(),
            })
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}", repr)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        hands.push(Hand::from_line(line));
    }
    hands
}

pub fn main() {
    let input = std::fs::read_to_string("data/input7.txt").unwrap();
    let mut hands = parse_input(&input);
    hands.sort();
    let result: usize = hands.iter().enumerate().map( |(i, c)| {
        // println!("{} {}", i+1, c);
        (i+1) * c.bid
    }).sum();
    println!("Day 7 Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_kind_compare() {
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn test_compare_hands() {
        let hand1 = Hand::from_line("2QQQA 220");
        let hand2 = Hand::from_line("2QQQK 483");
        assert!(hand1 > hand2);
    }
}
