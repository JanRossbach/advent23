use std::collections::HashSet;

pub struct Part {
    part_num: usize,
}

pub struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn get(&self, i: usize, j: usize) -> char {
        if i >= self.grid.len() || j >= self.grid[i].len() {
            return '.';
        }
        self.grid[i][j]
    }

    pub fn width(&self, i: usize) -> usize {
        self.grid[i].len()
    }
}

fn parse_input(input: &str) -> Grid {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
        height += 1;
        width = line.len();
    }
    Grid {
        grid,
        width,
        height,
    }
}

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && *c != '.' && *c != ' '
}

/// Extract the digits that are before the symbol at (i, j).
fn extract_digits_before(input: &Grid, i: usize, j: usize) -> String {
    let mut digits: String = String::new();
    let mut k = j;
    while k > 0 {
        k -= 1;
        let c = input.get(i, k);
        if c.is_digit(10) {
            digits = c.to_string() + digits.as_str();
        } else {
            break;
        }
    }
    digits
}

/// Extract the digits that are after the symbol at (i, j).
fn extract_digits_after(input: &Grid, i: usize, j: usize) -> String {
    let mut digits: String = String::new();
    let mut k = j;
    while k < input.width(i) {
        k += 1;
        let c = input.get(i, k);
        if c.is_digit(10) {
            digits = digits + c.to_string().as_str();
        } else {
            break;
        }
    }
    digits
}

/// Extract the number that the digit at (i, j) is part of. The digit can be at the beginning, middle or end of the number.
fn extract_num(input: &Grid, i: usize, j: usize) -> u32 {
    let digits_before = extract_digits_before(input, i, j);
    let digits_after = extract_digits_after(input, i, j);
    let num = digits_before + input.get(i, j).to_string().as_str() + digits_after.as_str();
    num.parse::<u32>().unwrap()
}

/// Find a digit that is adjacent to the symbol at (i, j) and return the number that it is part of.
/// The digit can be at the beginning, middle or end of the number.
/// The digit can be in any direction (up, down, left, right) and also diagonal.
fn find_part_nums(input: &Grid, i: usize, j: usize) -> Vec<u32> {
    let mut nums: HashSet<u32> = HashSet::new();
    let upper_left = input.get(i - 1, j - 1);
    if upper_left.is_digit(10) {
        nums.insert(extract_num(input, i - 1, j - 1));
    }
    let upper = input.get(i - 1, j);
    if upper.is_digit(10) {
        nums.insert(extract_num(input, i - 1, j));
    }
    let upper_right = input.get(i - 1, j + 1);
    if upper_right.is_digit(10) {
        nums.insert(extract_num(input, i - 1, j + 1));
    }
    let left = input.get(i, j - 1);
    if left.is_digit(10) {
        nums.insert(extract_num(input, i, j - 1));
    }
    let right = input.get(i, j + 1);
    if right.is_digit(10) {
        nums.insert(extract_num(input, i, j + 1));
    }
    let lower_left = input.get(i + 1, j - 1);
    if lower_left.is_digit(10) {
        nums.insert(extract_num(input, i + 1, j - 1));
    }
    let lower = input.get(i + 1, j);
    if lower.is_digit(10) {
        nums.insert(extract_num(input, i + 1, j));
    }
    nums.into_iter().collect::<Vec<u32>>()
}

pub fn extract_parts(input: &Grid) -> Vec<u32> {
    let mut parts: HashSet<u32> = HashSet::new();
    for (i, row) in input.grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if is_symbol(c) {
                let nums = find_part_nums(input, i, j);
                parts.extend(nums);
            }
        }
    }
    parts.into_iter().collect::<Vec<u32>>()
}

pub fn solve(input: &str, _star2: bool) -> u32 {
    let input = parse_input(input);
    let parts: Vec<u32> = extract_parts(&input);
    parts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*...\n..+.58.\n..592.....\n......755.\n...$.*....\n664.598..\n";

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
}
