use itertools::Itertools;

use crate::lib::{field::*, log};
pub fn solve(input: String) -> i32 {
    let input_vec = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let field = Field::from(input_vec).map(parse_item);
    let start = field.find_position(|item| *item == Item::Start).unwrap();
    let first = Direction::as_vec()
        .iter()
        .find(|dir| {
            field
                .get(&start.goto(dir))
                .unwrap_or(Item::Nil)
                .is_pointing(&dir.rev())
        })
        .unwrap()
        .to_owned();
    let mut current_pos = start;
    let mut current_dir = first;
    for i in 0.. {
        current_pos = current_pos.goto(&current_dir); // move pos
        current_dir = field // update dir
            .get(&current_pos)
            .unwrap_or(Item::Nil)
            .pointing_dirs()
            .iter()
            .find(|dir| **dir != current_dir.rev())
            .unwrap()
            .clone();
        if field.get(&current_pos).unwrap() == Item::Start {
            return (1 + i) / 2;
        }
    }
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
            Nil => vec![],
        }
    }
}
impl Pos {
    pub fn goto(&self, dir: &Direction) -> Pos {
        use Direction::*;
        match dir {
            Up => Pos(self.0 - 1, self.1),
            Down => Pos(self.0 + 1, self.1),
            Left => Pos(self.0, self.1 - 1),
            Right => Pos(self.0, self.1 + 1),
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
