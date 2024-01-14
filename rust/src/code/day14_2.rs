use itertools::Itertools;

use crate::utils::field::{Field, Pos};
const MAX_ITERATION: i32 = 1000000000;
// const MAX_ITERATION: i32 = 1;
const ROTATIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
// const ROTATIONS: [(i32, i32); 1] = [(-1, 0)];

pub fn solve(input: String) -> i32 {
    let mut field = Field::parse_str(input);
    let mut cache = Vec::new();
    let mut i = 0;
    let mut first = true;
    while i < MAX_ITERATION {
        for direction in ROTATIONS {
            roll_rocks(&mut field, direction);
        }
        if first && cache.iter().find(|f| f == &&field).is_some() {
            let match_len = i - cache.iter().find_position(|f| f == &&field).unwrap().0 as i32;
            while i < MAX_ITERATION - match_len {
                i += match_len;
            }
            first = false;
        }
        cache.push(field.clone());
        i += 1;
        println!("{i}th iter done")
    }
    return calculate_load(field); // O(ignorable)
}
fn roll_rocks(field: &mut Field<char>, direction: (i32, i32)) {
    let rocks = field.find_all_position(|char| char == &'O');
    for rock in rocks {
        // O(rocks)
        let mut old_pos = rock.clone();
        let mut pos = rock.clone();
        loop {
            // O(moving_dist)
            if let Some(new_pos) = move_pos(&pos, direction) {
                let new = field.get(&new_pos);
                if new == Some('.') {
                    old_pos = new_pos.clone();
                    pos = new_pos;
                } else if new == Some('O') {
                    pos = new_pos;
                    continue;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        field.set(&rock, '.');
        field.set(&old_pos, 'O');
    }
}
fn move_pos(pos: &Pos, direction: (i32, i32)) -> Option<Pos> {
    if (pos.0 == 0 && direction.0 == -1) || (pos.1 == 0 && direction.1 == -1) {
        return None;
    } else {
        Some(Pos(
            (pos.0 as i32 + direction.0) as usize,
            (pos.1 as i32 + direction.1) as usize,
        ))
    }
}
fn calculate_load(field: Field<char>) -> i32 {
    let all_o = field.find_all_position(|char| char == &'O');
    let height = field.height();
    all_o
        .iter()
        .map(|pos| height - pos.0)
        .map(|x| x as i32)
        .sum()
}
