use crate::utils::field::{Field, Pos};
pub fn solve(input: String, expansion_rate: i64) -> i128 {
    let field = Field::parse_str(input);
    // spread the galaxies
    let mut new = Field::new();
    let row_len = field.width();
    for row in field.rows() {
        if row.iter().all(|char| *char == '.') {
            new.push_row(vec!['*'; row_len]);
            // cannot use push_row_fill since in the first iteration theres no way to tell the len of each row (unless it's not empty)
        } else {
            new.push_row(row);
        }
    }
    let field = new;
    let mut new = Field::from(vec![vec![]; field.height()]);
    for col in field.cols() {
        if col.iter().all(|char| *char == '.' || *char == '*') {
            // dbg!(&col);
            new.push_col_fill('*'); // '*' represents expanded void
        } else {
            new.push_col(col);
        }
    }
    let field = new;
    // calculate dist. of each galaxies
    let mut sum_len: i128 = 0;
    let galaxies = field.find_all_position(|char| *char == '#');
    for pos1 in galaxies.iter() {
        for pos2 in galaxies.iter() {
            if pos1 != pos2 {
                sum_len += calculate_manhattan_dist(&field, &pos1, &pos2, expansion_rate) as i128;
            }
        }
    }
    return sum_len / 2; // each dist is calculated twice
}
pub fn calculate_manhattan_dist(
    field: &Field<char>,
    pos1: &Pos,
    pos2: &Pos,
    expansion_rate: i64,
) -> i64 {
    use std::cmp::{max, min};
    let vert_move = min(pos1.0, pos2.0)..(max(pos1.0, pos2.0)); // don't include last: last is included in hrz_move
    let hrz_move = min(pos1.1, pos2.1)..(max(pos1.1, pos2.1));
    let mut dist = 0;
    let y = pos1.1; // can be anywhere as long as the line is not empty (if empty, it means the line has expanded)
    for x in vert_move {
        match field.get(&Pos(x, y)) {
            Some('*') => dist += expansion_rate,
            _ => dist += 1,
        }
    }
    let x = pos1.0;
    for y in hrz_move {
        match field.get(&Pos(x, y)) {
            Some('*') => dist += expansion_rate,
            _ => dist += 1,
        }
    }
    return dist;
}
