use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameSet {
    cubes: HashMap<String, usize>,
}

impl GameSet {
    pub fn get(&self, color: &str) -> usize {
        *self.cubes.get(color).unwrap_or(&0)
    }

    pub fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        let mut possible = true;
        if self.get("red") > red {
            possible = false;
        }
        if self.get("green") > green {
            possible = false;
        }
        if self.get("blue") > blue {
            possible = false;
        }
        possible
    }

    pub fn power(&self) -> usize {
        let r = self.get("red");
        let g = self.get("green");
        let b = self.get("blue");
        r * g * b
    }
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

impl Game {
    pub fn from_line(line: &str) -> Game {
        let re = Regex::new(r"Game (\d+): (.*)").unwrap();
        let re2 = Regex::new(r"(\d+) (\w+)").unwrap();
        let id: usize = re.captures(line).unwrap()[1].parse().unwrap();
        let sets = re.captures(line).unwrap()[2]
            .split("; ")
            .map(|set| {
                let mut cubes = HashMap::new();
                for cap in re2.captures_iter(set) {
                    let color = cap[2].to_string();
                    let count: usize = cap[1].parse().unwrap();
                    cubes.insert(color, count);
                }
                GameSet { cubes }
            })
            .collect::<Vec<GameSet>>();
        Game { id, sets }
    }

    pub fn len(&self) -> usize {
        self.sets.len()
    }

    pub fn get(&self, i: usize) -> &GameSet {
        &self.sets[i]
    }

    pub fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        let mut possible = true;
        for set in &self.sets {
            if !set.is_possible(red, green, blue) {
                possible = false;
                break;
            }
        }
        possible
    }

    pub fn min_possible(&self) -> GameSet {
        let (r, g, b): (usize, usize, usize) =
            self.sets.iter().fold((0, 0, 0), |(r, g, b), set| {
                (
                    r.max(set.get("red")),
                    g.max(set.get("green")),
                    b.max(set.get("blue")),
                )
            });
        GameSet {
            cubes: [
                ("red".to_string(), r),
                ("green".to_string(), g),
                ("blue".to_string(), b),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}

pub fn parse_input(str: &str) -> Vec<Game> {
    str.lines()
        .map(|line| Game::from_line(&line))
        .collect::<Vec<Game>>()
}

pub fn find_solution(input: &str, exercise2: bool) -> usize {
    let games = parse_input(input);
    if !exercise2 {
        games
            .iter()
            .filter(|game| game.is_possible(12, 13, 14))
            .map(|game| game.id)
            .sum()
    } else {
        games.iter().map(|game| game.min_possible().power()).sum()
    }
}

pub fn solve(exercise2: bool) -> usize {
    let input = std::fs::read_to_string("data/input2.txt").unwrap();
    find_solution(&input, exercise2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";

    #[test]
    fn test_input_parse() {
        let parsed_input = parse_input(TEST_INPUT);
        assert_eq!(parsed_input.len(), 5);
        assert_eq!(parsed_input[0].get(0).get("red"), 4);
        assert_eq!(parsed_input[0].get(1).get("green"), 2);
    }

    #[test]
    fn test_day2() {
        let s = TEST_INPUT;
        assert_eq!(find_solution(s, false), 8);
        let s = solve(false);
        assert_eq!(s, 1931);
    }

    #[test]
    fn test_day2_exercise2() {
        let s = TEST_INPUT;
        assert_eq!(find_solution(s, true), 2286);
        let s = solve(true);
        assert_eq!(s, 83105);
    }
}
