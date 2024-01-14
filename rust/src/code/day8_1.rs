use itertools::Itertools;

pub fn solve(str: String) -> i32 {
    let mut input = str.lines().filter(|ln| !ln.is_empty());
    let mut instructions = input
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .cycle();
    let maze = input
        .map(|line| parse_into_node(line))
        .collect::<Vec<Node>>();
    let mut count = 0;
    let mut current = maze
        .iter()
        .find(|node| node.key == "AAA")
        .unwrap()
        .to_owned();
    loop {
        let target = current.dest[instructions.next().unwrap()].to_owned();
        current = maze
            .iter()
            .find(|node| node.key == target)
            .unwrap()
            .to_owned();
        count += 1;
        if current.key == "ZZZ" {
            break;
        }
    }
    count
}
#[derive(Clone)]

struct Node {
    key: String,
    dest: Vec<String>,
}
fn parse_into_node(line: &str) -> Node {
    let binding = line
        .replace("=", " ")
        .replace("(", " ")
        .replace(",", " ")
        .replace(")", " ");

    let entry = binding
        .split(" ")
        .filter(|str| !str.is_empty())
        .collect_vec();
    assert_eq!(entry.len(), 3);
    Node {
        key: entry[0].to_owned(),
        dest: vec![entry[1].to_owned(), entry[2].to_owned()],
    }
}
