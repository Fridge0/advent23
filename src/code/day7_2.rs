use itertools::Itertools;

pub fn solve(str: String) -> i64 {
    let entries = str.lines().map(|line| parse_line(line)).collect_vec();
    let mut entries = entries
        .into_iter()
        .map(|entry| score_entry(&entry))
        .collect_vec();
    entries.sort_by(|a, b| a.score.cmp(&b.score));
    let sorted = entries
        .iter()
        .enumerate()
        .map(|(rank, any)| (rank as i64 + 1, any))
        .inspect(|sth| println!("{:?}", sth))
        .map(|(rank, scored_entry)| (rank) * scored_entry.points);
    sorted.sum()
}
fn score_entry(entry: &Entry) -> ScoredEntry {
    let mut ratio = 1;
    let mut score: usize = 0;
    let possibles = duplicate_at_j(&entry.hands);
    for char in entry.hands.chars().rev() {
        score += match_char(char) * ratio;
        ratio *= 100;
    }
    score += possibles
        .iter()
        .map(|str| determine_type(&str) * ratio)
        .max()
        .unwrap();
    ScoredEntry {
        score,
        points: entry.points,
    }
}
fn duplicate_at_j(input: &str) -> Vec<String> {
    let mut possibilities = vec![input.to_owned()];
    for (i, char) in input.chars().enumerate() {
        if char == 'J' {
            possibilities = possibilities
                .iter()
                .map(|p| dup_at(p.to_owned(), i))
                .collect_vec()
                .concat();
        }
    }
    return possibilities;
}
fn dup_at(str: String, index: usize) -> Vec<String> {
    let pop = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"];
    return pop
        .iter()
        .map(|&char| {
            let mut str = str.clone();
            str.replace_range(index..(index + 1), char);
            str
        })
        .collect_vec();
}
fn determine_type(five_letter: &str) -> usize {
    let mut bucket = [0; 13];
    for char in five_letter.chars() {
        bucket[match_char(char) - 2] += 1;
    }
    let entry = bucket.into_iter().filter(|x| *x > 0).collect_vec();
    let &max = entry.iter().max().unwrap();
    if max == 5 {
        return 6; // Five of a kind
    } else if max == 4 {
        return 5; // Four of a kind
    } else if max == 3 {
        if entry.len() == 2 {
            return 4; // Full house
        } else {
            return 3; // Three of a kind
        }
    } else if max == 2 {
        if entry.len() == 3 {
            return 2; // Two pairs
        } else {
            return 1; // One pair
        }
    }
    return 0; // High card (bad)
}
fn match_char(char: char) -> usize {
    if let Some(num) = char.to_string().parse::<usize>().ok() {
        return num + 1;
    } else {
        match char {
            'J' => 2,
            'T' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("invalid char"),
        }
    }
}
struct Entry {
    hands: String,
    points: i64,
}
#[derive(Debug)]
struct ScoredEntry {
    points: i64,
    score: usize,
}
fn parse_line(line: &str) -> Entry {
    let mut entry = line.split(" ");
    let hands = entry.next().unwrap().to_string();
    let points = entry.next().unwrap().parse().unwrap();
    return Entry { hands, points };
}
