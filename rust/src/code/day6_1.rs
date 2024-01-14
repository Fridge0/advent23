use itertools::Itertools;

pub fn solve(str: String) -> i32 {
    let mut result = 1;
    if let [first_line, second_line] = &str
        .lines()
        .map(|line| line.split(" ").filter_map(|str| str.parse::<i32>().ok()))
        .collect_vec()[0..2]
    {
        for (time, dist) in first_line.clone().zip(second_line.clone().into_iter()) {
            let mut count = 0;
            for time_entry in 0..time {
                if (time - time_entry) * time_entry > dist {
                    count += 1;
                }
            }
            result *= count;
        }
        return result;
    }
    panic!();
}
