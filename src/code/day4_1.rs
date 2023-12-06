pub fn solve(input: String) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let numbers = line.split(":").collect::<Vec<&str>>();
        let mut numbers = vec![vec![numbers[0]], numbers[1].split("|").collect()].concat();
        numbers.remove(0);
        let numbers = numbers
            .iter()
            .map(|str| str.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let (win, have) = (numbers[0].to_owned(), numbers[1].to_owned());
        let mut count = 0;
        for number in win {
            if number.trim() == "" {
                continue;
            }
            if have.iter().any(|str| str.to_owned() == number) {
                count += 1;
            }
        }
        if count != 0 {
            total += (2 as i32).pow(count - 1);
        }
    }
    return total;
}
