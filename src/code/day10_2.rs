use itertools::Itertools;

use crate::lib::field::*;
pub fn solve(input: String) -> i32 {
    let input_vec = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let field = Field::from(input_vec.clone()).map(parse_item);
    let mut path = Field::from(input_vec).map(|_| false);
    let mut path_stack = Vec::new();
    let start = field
        .find_position(|item| *item == Item::Start)
        .expect("letter S was not found");
    let first_dir = Direction::as_vec()
        .iter()
        .find(|dir| {
            field
                .get(&start.goto(dir))
                .unwrap_or(Item::Nil)
                .is_pointing(&dir.rev()) // assure connection to S
        })
        .expect("paths around S is invalid")
        .to_owned();
    let mut current_pos = start;
    let mut current_dir = first_dir;
    let mut field = field;
    loop {
        // find path
        path.set(&current_pos, true);
        path_stack.push((current_pos.clone(), current_dir.clone()));
        current_pos = current_pos.goto(&current_dir); // move
        path_stack.push((current_pos.clone(), current_dir.clone()));
        // duplicate push is intentional. don't remove until u understand why this is needed (try remove either of them and run sample 2)
        // hint: at corners, how should you mark the front ("o" or "i")? how'd you know?
        current_dir = field
            .get(&current_pos)
            .expect("current pos does not exist(wtf)")
            .pointing_dirs()
            .iter()
            .find(|dir| **dir != current_dir.rev())
            .expect("the path is probably disconnected")
            .clone(); // update dir
                      // handle_head_ignorance_issue(&mut field, current_pos.clone(), prev_dir, current_dir);
        if field.get(&current_pos).unwrap() == Item::Start {
            path.set(&current_pos, true);
            break;
        }
    }
    let path = path;
    let mut count = 0;
    loop {
        // fill them with i/o
        if let Some((current_pos, current_dir)) = path_stack.pop() {
            let right = current_pos.goto(&current_dir.rev().rot());
            if path.get(&right) == Some(false) {
                field.set(&right, Item::Inside);
            }
            let left = current_pos.goto(&current_dir.rot());
            if path.get(&left) == Some(false) {
                field.set(&left, Item::Outside);
            }
        } else {
            break;
        }
    }
    loop_spread(&mut field, &path, Item::Outside);
    loop_spread(&mut field, &path, Item::Inside);
    let outside_count = field.find_all_position(|item| *item == Item::Outside).len();
    let inside_count = field.find_all_position(|item| *item == Item::Inside).len();
    let pathsize = path.find_all_position(|b| *b).len();
    println!("o: {}, i: {}", outside_count, inside_count);
    println!("{:?}", field);
    println!("path count: {pathsize}");
    assert_eq!(pathsize + outside_count + inside_count, field.len());
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
    fn angle(&self, other: &Direction) -> Direction {
        use Direction::*;
        fn dir_to_angle(dir: &Direction) -> i32 {
            match dir {
                Up => 0,
                Right => 1,
                Down => 2,
                Left => 3,
            }
        }
        let self_angle = dir_to_angle(self);
        let other_angle = dir_to_angle(other);
        let result = (other_angle - self_angle + 4) % 4;
        match result {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => panic!("%4 operator didn't work correctly i guess"),
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
