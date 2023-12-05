use regex::Regex;

#[derive(Debug)]
pub struct Seeds {
    pub seeds: Vec<usize>,
}

impl Seeds {
    pub fn from_entry(entry: &str) -> Seeds {
        let re = Regex::new(r"seeds: ([\d ]*)").unwrap();
        let caps = re.captures(entry).unwrap();
        let seeds = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(" ")
            .filter_map(|s| {
                s.trim()
                    .parse::<usize>()
                    .ok()
                    .and_then(|n| if n > 0 { Some(n) } else { None })
            })
            .collect::<Vec<usize>>();
        Seeds { seeds }
    }

    pub fn min_location_number(&self, maps: &Vec<Map>) -> usize {
        let mut min_location_number = usize::MAX;
        let n = self.seeds.len();
        for (i, seed) in self.seeds.iter().enumerate() {
            if i % 10000000 == 0 {
                // Show the number in % of total seeds processed
                println!("{}% done", i * 100 / n);
                println!("current min location number: {}", min_location_number);
            }
            let location_number = Seeds::location_number(*seed, maps);
            if location_number < min_location_number {
                min_location_number = location_number;
            }
        }
        min_location_number
    }

    pub fn location_number(seed: usize, maps: &Vec<Map>) -> usize {
        let mut number = seed;
        for map in maps {
            number = map.apply(number);
        }
        number
    }

    pub fn expand_ranges(self) -> Seeds {
        let mut seeds = Vec::new();
        let mut seded_iter = self.seeds.iter();
        assert!(self.seeds.len() % 2 == 0);
        while let Some(start) = seded_iter.next() {
            let length = seded_iter.next().unwrap();
            for i in *start..=*start + *length {
                seeds.push(i);
            }
        }
        Seeds { seeds }
    }
}

#[derive(Debug)]
pub struct Range {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl Range {
    pub fn in_range(&self, number: usize) -> bool {
        number >= self.source_start && number < self.source_start + self.length
    }

    pub fn apply(&self, number: usize) -> usize {
        if !self.in_range(number) {
            return number;
        }
        let offset = number - self.source_start;
        self.destination_start + offset
    }
}

#[derive(Debug)]
pub struct Map {
    pub source: String,
    pub destination: String,
    pub ranges: Vec<Range>,
}

impl Map {
    pub fn from_entry(entry: &str) -> Map {
        let re = Regex::new(r"([a-z]+)-to-([a-z]+) map:\n").unwrap();
        let caps = re.captures(entry).unwrap();

        let source = caps.get(1).unwrap().as_str().to_string();
        let destination = caps.get(2).unwrap().as_str().to_string();

        let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

        let mut ranges = Vec::new();

        for line in entry.lines().skip(1) {
            let caps = re.captures(line).unwrap();
            let destination_range_start = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let source_range_start = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let range_length = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

            ranges.push(Range {
                source_start: source_range_start,
                destination_start: destination_range_start,
                length: range_length,
            })
        }

        Map {
            source,
            destination,
            ranges,
        }
    }

    pub fn apply(&self, number: usize) -> usize {
        for range in &self.ranges {
            if range.in_range(number) {
                return range.apply(number);
            }
        }
        number
    }
}

pub fn parse_input(input: &str) -> (Seeds, Vec<Map>) {
    let mut maps = Vec::new();
    let mut input_iter = input.split("\n\n");
    let first_line = input_iter.next().unwrap();
    let seeds = Seeds::from_entry(first_line);

    for entry in input_iter {
        let map = Map::from_entry(entry);
        maps.push(map);
    }
    (seeds, maps)
}

pub fn solve(input: &str) -> usize {
    let (seeds, maps) = parse_input(input);
    seeds.min_location_number(&maps)
}

pub fn solve_star2(input: &str) -> usize {
    let (seeds, maps) = parse_input(input);

    let seeds = seeds.expand_ranges();
    println!("{} seeds expanded!", seeds.seeds.len());
    println!("Finding the minimum location number...");

    seeds.min_location_number(&maps)
}

pub fn main() {
    // let input = std::fs::rad_to_string("data/input5.txt").unwrap();
    // println!("Day 5: {}", solve_star2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_day5() {
        let input = std::fs::read_to_string("data/test_input5.txt").unwrap();
        let (_seeds, maps) = parse_input(&input);
        assert_eq!(maps.len(), 7);
        assert_eq!(maps[0].apply(1), 1);
        assert_eq!(maps[0].apply(49), 49);
        assert_eq!(maps[0].apply(50), 52);
        assert_eq!(maps[0].apply(98), 50);
        assert_eq!(maps[0].apply(99), 51);
    }

    #[test]
    fn test_star1_test_input_day5() {
        let input = std::fs::read_to_string("data/test_input5.txt").unwrap();
        assert_eq!(solve(&input), 35);
    }

    #[test]
    fn test_star1_real_input_day5() {
        let input = std::fs::read_to_string("data/input5.txt").unwrap();
        assert_eq!(solve(&input), 51580674);
    }

    #[test]
    fn test_star2_test_input() {
        let input = std::fs::read_to_string("data/test_input5.txt").unwrap();
        assert_eq!(solve_star2(&input), 46);
    }
}
