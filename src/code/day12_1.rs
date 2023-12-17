use itertools::{Itertools, PeekingNext};

pub fn solve(input: String) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        // O(input.lines().count())
        let (line, pattern) = parse_line(line);
        println!("{:?}", &pattern);
        let question_marks = line.chars().filter(|char| *char == '?').count();
        let sequence = SequenceCharGen::new(question_marks);
        for char_set in sequence {
            // O(2^line.len())
            let mut char_set = char_set.iter();
            let mut new = String::new();
            for char in line.chars() {
                // O(line.len())
                if char == '?' {
                    new.push(*char_set.next().unwrap());
                } else {
                    new.push(char);
                }
            }
            if is_valid(&new, &pattern) {
                total += 1;
            }
        }
    }
    return total;
}
fn parse_line(line: &str) -> (String, Vec<i32>) {
    let mut pattern = line.split(" ");
    let line = pattern
        .next()
        .expect("includes empty line. solution: remove it");
    let pattern = pattern
        .next()
        .unwrap()
        .split(",")
        .filter_map(|str| str.parse::<i32>().ok())
        .collect();
    return (line.to_owned(), pattern);
}
fn is_valid(line: &String, pattern: &Vec<i32>) -> bool {
    let count = line
        .split(".")
        .map(|str| str.len() as i32)
        .filter(|i| i != &0)
        .collect_vec();
    return pattern.len() == count.len() && pattern.iter().zip(count.iter()).all(|(a, b)| a == b);
}
pub fn test_is_valid(line: &str, pattern: Vec<i32>) {
    println!("testing: {line} with pattern {pattern:?}");
    let result = is_valid(&line.to_string(), &pattern);
    dbg!(result);
}
struct SequenceCharGen {
    len: usize,
    current: i32,
    total: i32,
}
impl SequenceCharGen {
    fn new(len: usize) -> Self {
        Self {
            len,
            current: 0,
            total: (2 as i32).pow(len as u32),
        }
    }
    fn len(&self) -> usize {
        self.len
    }
}
impl Iterator for SequenceCharGen {
    type Item = Vec<char>;
    fn next(&mut self) -> Option<Vec<char>> {
        if self.current >= self.total {
            return None;
        }
        let result = Some(
            fmt_binary(self.current, self.len)
                .replace("0", ".")
                .replace("1", "#")
                .chars()
                .collect_vec(),
        );
        self.current += 1;
        result
    }
}
fn fmt_binary(value: i32, len: usize) -> String {
    let mut value = value;
    let mut result = String::new();
    let two: i32 = 2;
    for i in (0..len).rev() {
        let i = i as u32;
        if value >= two.pow(i) {
            value -= two.pow(i);
            result += "1";
        } else {
            result += "0";
        }
    }
    result
}
