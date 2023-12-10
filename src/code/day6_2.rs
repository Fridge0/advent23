use itertools::Itertools;

pub fn solve(str: String) -> i64 {
    let mut result = 1;
    if let [time, dist] = &str
        .lines()
        .map(|line| {
            line.chars()
                .filter(|char| char.to_string().parse::<i64>().is_ok())
                .fold(String::new(), |str, char| str + &char.to_string())
                .parse::<i64>()
                .unwrap()
        })
        .collect_vec()[0..2]
    {
        let mut count = 0;
        for time_entry in 0..*time {
            if (time - time_entry) * time_entry > *dist {
                count += 1;
            }
        }
        result *= count;
        return result;
    }
    panic!();
}
