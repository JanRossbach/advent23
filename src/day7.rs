use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Card {
    pub rank: u8,
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
}

impl Hand {
    pub fn four_of_a_kind(&self) -> Option<u8> {
        let mut ranks = [0; 15];
        for card in self.cards.iter() {
            ranks[card.rank as usize] += 1;
        }
        for (rank, count) in ranks.iter().enumerate() {
            if *count == 4 {
                return Some(rank as u8);
            }
        }
        None
    }

    pub fn full_house(&self) -> Option<u8> {
        let mut ranks = [0; 15];
        for card in self.cards.iter() {
            ranks[card.rank as usize] += 1;
        }
        let mut three_of_a_kind = None;
        let mut two_of_a_kind = None;
        for (rank, count) in ranks.iter().enumerate() {
            if *count == 3 {
                three_of_a_kind = Some(rank as u8);
            } else if *count == 2 {
                two_of_a_kind = Some(rank as u8);
            }
        }
        if three_of_a_kind.is_some() && two_of_a_kind.is_some() {
            return three_of_a_kind;
        }
        None
    }

    pub fn three_of_a_kind(&self) -> Option<u8> {
        let mut ranks = [0; 15];
        for card in self.cards.iter() {
            ranks[card.rank as usize] += 1;
        }
        for (rank, count) in ranks.iter().enumerate() {
            if *count == 3 {
                return Some(rank as u8);
            }
        }
        None
    }

    pub fn two_of_a_kind(&self) -> Option<u8> {
        let mut ranks = [0; 15];
        for card in self.cards.iter() {
            ranks[card.rank as usize] += 1;
        }
        for (rank, count) in ranks.iter().enumerate() {
            if *count == 2 {
                return Some(rank as u8);
            }
        }
        None
    }

    pub fn two_pair_high(&self) -> Option<u8> {
        let mut ranks = [0; 15];
        for card in self.cards.iter() {
            ranks[card.rank as usize] += 1;
        }
        let mut high_pair = None;
        let mut low_pair = None;
        for (rank, count) in ranks.iter().enumerate() {
            if *count == 2 {
                if high_pair.is_none() {
                    high_pair = Some(rank as u8);
                } else {
                    low_pair = Some(rank as u8);
                }
            }
        }
        if high_pair.is_some() && low_pair.is_some() {
            return Some(high_pair.unwrap());
        }
        None
    }

    pub fn high_card(&self) -> u8 {
        let mut high_card = 0;
        for card in self.cards.iter() {
            if card.rank > high_card {
                high_card = card.rank;
            }
        }
        high_card
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cards == other.cards;
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut self_ranks = [0; 15];
        let mut other_ranks = [0; 15];
        if let Some(rank) = self.four_of_a_kind() {
            if let Some(other_rank) = other.four_of_a_kind() {
                return Some(rank.cmp(&other_rank));
            }
        }

        return Some(Ordering::Equal);
    }
}

impl Ord for Hand {
    pub fn cmp(&self, other) {

    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    let re = regex::Regex::new(r"([2-9TJQKA]{5}) (\d+)").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let cards = captures[1]
            .chars()
            .map(|c| Card::from_char(c))
            .collect::<Vec<_>>();
        let bid = captures[2].parse::<usize>().unwrap();
        hands.push(Hand { cards , bid });
    }
    hands
}

pub fn main() {
    let input = std::fs::read_to_string("data/test_input7.txt").unwrap();
    let hands = parse_input(&input);
    println!("Hands: {:?}", hands);
}
