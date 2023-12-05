use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Part {
    part_num: usize,
    len: usize,
    i: usize,
    j: usize,
}

pub struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn get(&self, i: usize, j: usize) -> char {
        if i >= self.grid.len() || j >= self.grid[i].len() {
            return '.';
        }
        self.grid[i][j]
    }

    pub fn get_signed(&self, i: i32, j: i32) -> char {
        if i < 0 || j < 0 {
            return '.';
        }
        let (i, j) = (i as usize, j as usize);
        self.get(i, j)
    }

    pub fn width(&self, i: usize) -> usize {
        self.grid[i].len()
    }
}

fn parse_input(input: &str) -> Grid {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    Grid { grid }
}

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}

fn symbol_adjacent(input: &Grid, p: &Part) -> bool {
    let mut adjacent = false;
    let (i, j) = (p.i, p.j);
    for j in j..=j + p.len {
        let (i, j): (i32, i32) = (i as i32, j as i32);
        let upper_left = input.get_signed(i - 1, j - 1);
        let upper = input.get_signed(i - 1, j);
        let upper_right = input.get_signed(i - 1, j + 1);
        let left = input.get_signed(i, j - 1);
        let right = input.get_signed(i, j + 1);
        let lower_left = input.get_signed(i + 1, j - 1);
        let lower = input.get_signed(i + 1, j);
        let lower_right = input.get_signed(i + 1, j + 1);
        if is_symbol(&upper_left)
            || is_symbol(&upper)
            || is_symbol(&upper_right)
            || is_symbol(&left)
            || is_symbol(&right)
            || is_symbol(&lower_left)
            || is_symbol(&lower)
            || is_symbol(&lower_right)
        {
            adjacent = true;
        }
    }
    adjacent
}

pub fn extract_parts(input: &Grid) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in input.grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            //println!("c:{} i:{} j:{}", *c, i, j);
            if c.is_digit(10) && !seen.contains(&(i, j)) {
                let mut num: String = String::new();
                num.push(*c);
                let (k, l) = (i, j);
                let mut len = 1;
                while input.get(k, l + len).is_digit(10) {
                    seen.insert((k, l + len));
                    num.push(input.get(k, l + len));
                    len += 1;
                }
                parts.push(Part {
                    part_num: num.parse().unwrap(),
                    len,
                    i,
                    j,
                });
            }
        }
    }
    parts
}

pub fn solve(input: &str, _star2: bool) -> u32 {
    let input = parse_input(input);
    let parts: Vec<Part> = extract_parts(&input);
    let parts = parts
        .iter()
        .filter(|part| symbol_adjacent(&input, part))
        .collect::<HashSet<&Part>>();

    parts.iter().map(|part| part.part_num as u32).sum()
}



#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n......+.58.\n..592.....\n......755.\n...$.*....\n664.598..\n";

    #[test]
    fn test_parse_input() {
        let g: Grid = parse_input(TEST_INPUT);
    }

    #[test]
    fn test_test_input_star1() {
        let real_input = std::fs::read_to_string("data/input3.txt").unwrap();
        assert_eq!(solve(TEST_INPUT, false), 4361);
        assert_eq!(solve(&real_input, false), 0);
    }

    #[test]
    fn test_star2() {
        let real_input = std::fs::read_to_string("data/input3.txt").unwrap();
        assert_eq!(solve(TEST_INPUT, true), 467835);
        assert_eq!(solve(&real_input, true), 77509019);
    }

    #[test]
    fn test_is_adjacent() {
        let input = parse_input(TEST_INPUT);
        assert!(symbol_adjacent(
            &input,
            &Part {
                part_num: 592,
                len: 3,
                i: 6,
                j: 2,
            }
        ));
    }
}
