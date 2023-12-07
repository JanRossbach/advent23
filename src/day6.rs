use regex::Regex;

#[derive(Clone, Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {

    pub fn new(time: usize, record: usize) -> Self {
        Self {
            time,
            record,
        }
    }

    pub fn calc_distance(&self, charge_time: usize) -> usize {
        let remaining_time = self.time - charge_time;
        let distance = remaining_time * charge_time;
        distance
    }

    pub fn beats_record(&self, charge_time: usize) -> bool {
        let distance = self.calc_distance(charge_time);
        distance > self.record
    }

    pub fn num_ways_to_beat_record(&self) -> usize {
        (1..self.time - 1)
            .filter(|&charge_time| self.beats_record(charge_time))
            .count()
    }

    pub fn extend_with_race(&self, other: &Race) -> Self {
        // concatinate the digits of the two numbers
        if self.time == 0 {
            return other.clone();
        }
        let new_time = format!("{}{}", self.time, other.time).parse::<usize>().unwrap();
        let new_record = format!("{}{}", self.record, other.record).parse::<usize>().unwrap();
        Self {
            time: new_time,
            record: new_record,
        }
    }
}

//
fn parse_input(input: &str) -> Vec<Race> {
    let first_line = input.lines().next().unwrap();
    let second_line = input.lines().nth(1).unwrap();
    let times = first_line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let records = second_line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut races = Vec::new();
    for (&time, &record) in times.iter().zip(records.iter()) {
        races.push(Race { time, record })
    }
    races
}

pub fn main() {
    let input = std::fs::read_to_string("data/input6.txt").unwrap();
    println!("Result Day6: {}", solve6_star2(&input));
}

pub fn solve6(input: &str) -> usize {
    let races: Vec<Race> = parse_input(&input);
    races.iter().map(|r| r.num_ways_to_beat_record()).product()
}

pub fn solve6_star2(input: &str) -> usize {
    let races = parse_input(&input);
    let race: Race = races.iter().fold(Race::new(0,0), |a, e| a.extend_with_race(e));
    race.num_ways_to_beat_record()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input6() {
        let input = std::fs::read_to_string("data/test_input6.txt").unwrap();
        let races: Vec<Race> = parse_input(&input);
        assert_eq!(races.len(), 3);
    }

    #[test]
    fn solve_day6_test() {
        let input = std::fs::read_to_string("data/test_input6.txt").unwrap();
        let result = solve6(&input);
        assert_eq!(result, 288);
    }

    #[test]
    fn solve_day6_real() {
        let input = std::fs::read_to_string("data/input6.txt").unwrap();
        let result = solve6(&input);
        assert_eq!(result, 5133600);
    }

    #[test]
    fn solve_day6_star2_test() {
        let input = std::fs::read_to_string("data/test_input6.txt").unwrap();
        let result = solve6_star2(&input);
        assert_eq!(result, 71503);
    }

    #[test]
    fn solve_day6_star2_real() {
        let input = std::fs::read_to_string("data/input6.txt").unwrap();
        let result = solve6_star2(&input);
        assert_eq!(result, 40651271);
    }
}
