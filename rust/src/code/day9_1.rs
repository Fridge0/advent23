use itertools::Itertools;

pub fn solve(str: String) -> i32 {
    let mut result = 0;
    for line in str.lines() {
        let mut parse = Vec::new();
        let mut line = line
            .split(" ")
            .filter_map(|str| str.parse::<i32>().ok())
            .collect_vec();
        parse.push(line.clone());
        loop {
            line = differentiate(line);
            parse.push(line.clone());
            if all_zero(&line) {
                break;
            }
        }
        let mut prev_last = 0;
        for line in parse.iter_mut().rev() {
            line.push(prev_last + line.last().unwrap());
            prev_last = *line.last().unwrap();
        }
        let expected_value = prev_last;
        result += expected_value;
    }
    return result;
}

fn differentiate(line: Vec<i32>) -> Vec<i32> {
    line.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}
fn all_zero(line: &Vec<i32>) -> bool {
    line.iter().all(|i| *i == 0)
}
