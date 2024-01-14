use crate::utils::field::{Field, Pos};

pub fn solve(input: String) -> i32 {
    let mut field = Field::parse_str(input);
    let all_pos = field.find_all_position(|_| true);
    loop {
        let some_remains = all_pos.iter().any(|pos| move_up(&mut field, pos));
        if !some_remains {
            break;
        }
    }
    dbg!(&field);
    return calculate_load(field);
}
fn move_up(field: &mut Field<char>, pos: &Pos) -> bool {
    if pos.0 == 0 {
        return false;
    }
    if field.get(&pos) != Some('O') {
        return false;
    }
    let up = Pos(pos.0 - 1, pos.1);
    if field.get(&up) == Some('.') {
        field.set(&pos, '.');
        field.set(&up, 'O');
        return true;
    }
    return false;
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
