use itertools::Itertools;

use crate::lib::field::*;
pub fn solve(input: String) -> i32 {
    let mut outside_count = 0;
    let mut inside_count = 0;
    let input_vec = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let field = Field::from(input_vec.clone()).map(parse_item);
    let mut path = Field::from(input_vec).map(|_| false);
    let mut path_stack = Vec::new();
    let start = field.find_position(|item| *item == Item::Start).unwrap();
    let first = Direction::as_vec()
        .iter()
        .find(|dir| {
            field
                .get(&start.goto(dir))
                .unwrap_or(Item::Nil)
                .is_pointing(&dir.rev())
        })
        .expect("S not found")
        .to_owned();
    let mut current_pos = start;
    let mut current_dir = first;
    loop {
        // find path
        path.set(&current_pos, true);
        path_stack.push((current_pos.clone(), current_dir.clone()));
        current_pos = current_pos.goto(&current_dir); // move
        current_dir = field
            .get(&current_pos)
            .expect("current pos does not exist(wtf)")
            .pointing_dirs()
            .iter()
            .find(|dir| **dir != current_dir.rev())
            .unwrap()
            .clone(); // update dir
        path.set(&current_pos, true);
        if field.get(&current_pos).unwrap() == Item::Start {
            break;
        }
    }
    let mut field = field;
    loop {
        // fill them with i/o
        if let Some((current_pos, current_dir)) = path_stack.pop() {
            let right = current_pos.goto(&current_dir.rev().rot());
            if path.get(&right) == Some(false) {
                field.set(&right, Item::Inside);
                path.set(&right, true);
                inside_count += 1;
            }
            let left = current_pos.goto(&current_dir.rot());
            if path.get(&left) == Some(false) {
                field.set(&left, Item::Outside);
                path.set(&left, true);
                outside_count += 1;
            }
            if field.get(&current_pos).unwrap() == Item::Start {
                break;
            }
        } else {
            break;
        }
    }
    loop_spread(&mut field, &path, Item::Outside);
    loop_spread(&mut field, &path, Item::Inside);
    let outside_count = field.find_all_position(|item| *item == Item::Outside).len();
    let inside_count = field.find_all_position(|item| *item == Item::Inside).len();
    // println!("{:?}", field);
    println!("o: {}, i: {}", outside_count, inside_count);
    return 0;
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum Item {
    Start,
    UD,
    LR,
    UL,
    UR,
    DL,
    DR,
    Inside,
    Outside,
    Nil,
}
impl Item {
    fn is_pointing(&self, dir: &Direction) -> bool {
        use Item::*;
        let cur = self.clone();
        if cur == Start {
            return true;
        }
        self.pointing_dirs()
            .iter()
            .find(|dir2| dir == *dir2)
            .is_some()
    }
    fn pointing_dirs(&self) -> Vec<Direction> {
        use Direction::*;
        use Item::*;
        match self {
            Start => vec![Up, Down, Left, Right],
            UD => vec![Up, Down],
            LR => vec![Left, Right],
            UL => vec![Up, Left],
            UR => vec![Up, Right],
            DL => vec![Down, Left],
            DR => vec![Down, Right],
            _ => vec![],
        }
    }
}
impl std::fmt::Display for Item {
    fn fmt(&self, ftr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Item::*;
        write!(
            ftr,
            "{}",
            match self {
                Start => "S",
                UD => "|",
                LR => "-",
                UL => "┘",
                UR => "└",
                DL => "┐",
                DR => "┌",
                Inside => "i",
                Outside => "o",
                _ => ".",
            }
        )?;
        return Ok(());
    }
}
impl Pos {
    pub fn from(x: i32, y: i32) -> Self {
        Pos(x as usize, y as usize)
    }
    pub fn goto(&self, dir: &Direction) -> Pos {
        let (zero, one) = (self.0 as i32, self.1 as i32);
        use Direction::*;
        match dir {
            Up => Pos::from(zero - 1, one),
            Down => Pos::from(zero + 1, one),
            Left => Pos::from(zero, one - 1),
            Right => Pos::from(zero, one + 1),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn as_vec() -> Vec<Self> {
        use Direction::*;
        return vec![Up, Right, Down, Left];
    }
    fn rev(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
    fn rot(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}
fn parse_item(char: char) -> Item {
    use Item::*;
    match char {
        'S' => Start,
        '|' => UD,
        '-' => LR,
        'J' => UL,
        'L' => UR,
        '7' => DL,
        'F' => DR,
        '.' => Nil,
        c => panic!("ParseError: found {c}."),
    }
}
fn loop_spread(field: &mut Field<Item>, path: &Field<bool>, target_item: Item) -> i32 {
    let mut has_told = false;
    let mut count = 0;
    loop {
        let current = field.find_all_position(|item| *item == target_item);
        let targets = current
            .iter()
            .map(|pos| {
                Direction::as_vec()
                    .iter()
                    .map(|dir| pos.goto(&dir))
                    .inspect(|pos| {
                        if field.get(&pos).is_none() && !has_told {
                            has_told = true;
                            println!("{target_item} is the outside");
                        }
                    })
                    .filter(|pos| {
                        path.get(&pos) == Some(false) && field.get(&pos) != Some(target_item)
                    })
                    .collect_vec()
            })
            .concat();
        if targets.len() == 0 {
            break;
        }
        for target in targets.iter() {
            field.set(target, target_item);
            count += 1;
        }
    }
    count
}
