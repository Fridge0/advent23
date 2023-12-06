use super::super::lib;

pub fn main() {
    let target = lib::read_file("../txt/day1-main.txt");
    println!("{}", solve(target));
}

pub fn solve(target: String) -> i32 {
    let result = target
        .lines()
        .map(|line| -> (i32, i32) {
            let mut first = 0;
            let mut last = 0;
            for char in line.chars() {
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

fn parse_int(char: char) -> Option<i32> {
    char.to_string().parse::<i32>().ok()
}
