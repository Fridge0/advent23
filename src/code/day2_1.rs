use std::fs;
pub fn main() {
    let path = "../source/day2-main.txt".to_string();
    let src = fs::read_to_string(path).unwrap();
    println!("{}", solve(src));
}
const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn _test() {
    let path = "../day2-sample.txt".to_string();
    let src = fs::read_to_string(path).unwrap();
    let result = solve(src);
    assert_eq!(8, result);
}

pub fn solve(src: String) -> i32 {
    let lines = src.lines();
    let split_lines = lines.map(|line| line.split(":"));
    let split_lines: Vec<Line> = split_lines
        .map(|line| line.map(|slice| slice.to_string()).collect())
        .map(|slice| Line::from_slice(slice))
        .collect();
    let split_lines: Vec<ParsedLine> = split_lines
        .iter()
        .map(|line| ParsedLine::from_line(line))
        .collect();
    let mut total: i32 = 0;
    for line in split_lines {
        if line.games.iter().all(|game| game.satisfies()) {
            total += line.count as i32;
        }
    }
    return total;
}

#[derive(Debug, Clone)]
struct Line {
    count: String,
    games: Vec<String>,
}
impl Line {
    fn from_slice(slice: Vec<String>) -> Self {
        Line {
            count: slice[0].clone(),
            games: slice[1]
                .split(";")
                .map(|str| str.trim().to_string())
                .collect(),
        }
    }
}
#[derive(Debug)]
struct ParsedLine {
    count: usize,
    games: Vec<Game>,
}

impl ParsedLine {
    fn from_line(line: &Line) -> Self {
        ParsedLine {
            count: line.count.trim().to_string().only_number()[0] as usize,
            games: line.games.iter().map(|game| Game::from_str(game)).collect(),
        }
    }
}
#[derive(Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}
impl Game {
    fn from_str(str: &str) -> Self {
        let atoms = str.split(", ").map(|atom| {
            (
                atom.to_string().only_number()[0],
                atom.split(" ").nth(1).unwrap_or(""),
            )
        });
        Game {
            red: atoms
                .clone()
                .find(|atom| atom.1 == "red")
                .unwrap_or((0, ""))
                .0,
            green: atoms
                .clone()
                .find(|atom| atom.1 == "green")
                .unwrap_or((0, ""))
                .0,
            blue: atoms
                .clone()
                .find(|atom| atom.1 == "blue")
                .unwrap_or((0, ""))
                .0,
        }
    }
    fn satisfies(&self) -> bool {
        self.blue <= MAX_BLUE && self.green <= MAX_GREEN && self.red <= MAX_RED
    }
}
trait OnlyNumber {
    fn only_number(&self) -> Vec<i32>;
}
impl OnlyNumber for String {
    fn only_number(&self) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = Vec::new();
        for char in self.chars() {
            if let Ok(n) = char.to_string().parse::<i32>() {
                stack.push(n);
            } else if stack.len() != 0 {
                result.push(concat_vec_int(&stack));
                stack.clear();
            }
        }
        if !stack.is_empty() {
            result.push(concat_vec_int(&stack))
        }
        return result;
    }
}

fn concat_vec_int(vec: &Vec<i32>) -> i32 {
    vec.iter()
        .map(|int| int.to_string())
        .collect::<Vec<String>>()
        .concat()
        .parse()
        .unwrap()
}
