use std::fs;

pub fn main() {
    let path = "../txt/day1-main.txt".to_string();
    let target = fs::read_to_string(path).unwrap();
    let result = solve(target);
    println!("{}", result);
}

pub fn solve(target: String) -> i32 {
    let result = target
        .lines()
        .map(|line| -> String {
            let mut line = line.chars().collect::<Vec<char>>();
            line.extend([' ', ' ', ' ', ' ', ' '].iter());
            parse_line(line)
        })
        .map(|line| -> (i32, i32) {
            let mut first = 0;
            let mut last = 0;
            for char in line.chars() {
                if char == ' ' {
                    continue;
                }
                match parse_int(char) {
                    Some(num) => {
                        first = num;
                        break;
                    }
                    None => continue,
                }
            }
            for char in line.chars().rev() {
                match parse_int(char) {
                    Some(num) => {
                        last = num;
                        break;
                    }
                    None => continue,
                }
            }
            (first, last)
        })
        .map(|(a, b)| a * 10 + b)
        .sum();
    result
}

fn parse_line(line: Vec<char>) -> String {
    let mut line = line;
    for i in 0..line.len() {
        match match_num(i, &line) {
            Some(num) => line[i] = num,
            None => (),
        }
    }
    let result = line
        .iter()
        .map(|char| char.to_string())
        .collect::<Vec<String>>()
        .concat();
    result
}

fn parse_int(char: char) -> Option<i32> {
    char.to_string().parse::<i32>().ok()
}

fn match_num(i: usize, vec: &Vec<char>) -> Option<char> {
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .map(|str| str.to_string())
    .enumerate();
    let target = (i..i + 5).into_iter().map(|i| get_fallible(i, vec));
    for (num, num_str) in numbers {
        let result = num_str
            .to_string()
            .chars()
            .into_iter()
            .zip(target.clone().into_iter())
            .all(|(a, b)| Some(a) == b);
        if result {
            return Some(num.to_string().chars().nth(0).unwrap());
        }
    }
    return None;
}

fn get_fallible<T: Clone>(index: usize, vec: &Vec<T>) -> Option<T> {
    if index < vec.len() {
        return Some(vec[index].clone());
    } else {
        return None;
    }
}
