fn find_digits(line: &str) -> u32 {
    let mut digits: Vec<char> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        match c {
            '0'..='9' => digits.push(c.clone()),
            'o' => {
                if line.len() >= i + 3 {
                    let s = &line[i..i + 3];
                    if s == "one" {
                        digits.push('1');
                    }
                }
            }
            't' => {
                if line.len() >= i + 3 {
                    let s = &line[i..i + 3];
                    if s == "two" {
                        digits.push('2');
                    }
                }
                if line.len() >= i + 5 {
                    let s = &line[i..i + 5];
                    if s == "three" {
                        digits.push('3');
                    }
                }
            }
            'e' => {
                if line.len() >= i + 5 {
                    let s = &line[i..i + 5];
                    if s == "eight" {
                        digits.push('8');
                    }
                }
            }
            'f' => {
                if line.len() >= i + 4 {
                    let s = &line[i..i + 4];
                    if s == "five" {
                        digits.push('5');
                    } else if s == "four" {
                        digits.push('4');
                    }
                }
            }
            'n' => {
                if line.len() >= i + 4 {
                    let s = &line[i..i + 4];
                    if s == "nine" {
                        digits.push('9');
                    }
                }
            }
            's' => {
                if line.len() >= i + 5 {
                    let s = &line[i..i + 5];
                    if s == "seven" {
                        digits.push('7');
                    }
                }
                if line.len() >= i + 3 {
                    let s = &line[i..i + 3];
                    if s == "six" {
                        digits.push('6');
                    }
                }
            }
            _ => (),
        }
    }
    let first_digit = digits[0];
    let last_digit = digits[digits.len() - 1];
    let combined_num: &str = &format!("{}{}", first_digit, last_digit);
    combined_num.parse::<u32>().unwrap()
}

pub fn find_solution(s: &str) -> u32 {
    let mut sum = 0;
    for line in s.lines() {
        let combined_num = find_digits(line);
        sum += combined_num;
    }
    sum
}
