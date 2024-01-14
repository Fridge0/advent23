use itertools::Itertools;

pub fn solve(str: String) -> u128 {
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
    let mut currents: Vec<Node> = maze
        .iter()
        .filter(|node| node.key.ends_with("A"))
        .map(|node| node.to_owned())
        .collect();
    let mut is_complete = vec![None; currents.len()];
    println!("{:?} many found", currents.len());
    loop {
        count += 1;
        let choice = instructions.next().unwrap();
        for (idx, current) in currents.iter_mut().enumerate() {
            let target = current.dest[choice].to_owned();
            *current = maze
                .iter()
                .find(|node| node.key == target)
                .unwrap()
                .to_owned();
            if current.key.ends_with("Z") {
                if is_complete[idx].is_none() {
                    println!("iteration: {count}");
                    is_complete[idx] = Some(count);
                }
            }
        }
        if is_complete.iter().all(|complete| complete.is_some()) {
            println!("{:?}", is_complete.iter().map(|num| num.unwrap()));
            break is_complete
                .iter()
                .map(|num| num.unwrap())
                .fold(1, num::integer::lcm);
        }
    }
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
