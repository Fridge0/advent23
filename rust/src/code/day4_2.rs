use crate::lib::tryvec;
pub fn solve(input: String) -> i32 {
    let mut wins = vec![0; input.lines().count()];

    for (line_count, line) in input.lines().enumerate() {
        let mut winning = 0;
        let numbers = line.split(":").collect::<Vec<&str>>();
        let numbers = vec![vec![numbers[0]], numbers[1].split("|").collect()].concat();
        let mut numbers = numbers
            .iter()
            .map(|str| str.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        numbers.remove(0);
        let (win, have) = (numbers[0].to_owned(), numbers[1].to_owned());
        for number in win {
            if number.trim() == "" {
                continue;
            }
            if have.iter().any(|str| str.to_owned() == number) {
                winning += 1;
            }
        }
        wins[line_count] += winning;
    }

    let mut cards = vec![1; input.lines().count()];
    for count in 0..cards.len() {
        for i in count + 1..=count + wins[count] {
            if i < cards.len() {
                cards[i] += cards[count];
            }
        }
    }
    return cards.iter().sum();
}
