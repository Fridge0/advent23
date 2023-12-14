use crate::utils::field::{Field, Pos};
pub fn solve(input: String) -> i32 {
    let field = Field::parse_str(input);
    // spread the galaxies
    let mut new = Field::new();
    let row_len = field.width();
    for row in field.rows() {
        if row.iter().all(|char| *char == '.') {
            new.push_row(vec!['.'; row_len]);
        }
        new.push_row(row);
    }
    let field = new;
    let mut new = Field::from(vec![vec![]; field.height()]);
    for col in field.cols() {
        if col.iter().all(|char| *char == '.') {
            // dbg!(&col);
            new.push_col_fill('.'); // '*' represents expanded void
        }
        new.push_col(col);
    }
    let field = new;
    // calculate dist. of each galaxies
    let mut sum_len = 0;
    let galaxies = field.find_all_position(|char| *char == '#');
    for pos1 in galaxies.iter() {
        for pos2 in galaxies.iter() {
            sum_len += calculate_manhattan_dist(&pos1, &pos2);
        }
    }
    return sum_len / 2; // each dist is calculated twice
}
fn calculate_manhattan_dist(pos1: &Pos, pos2: &Pos) -> i32 {
    let pos1 = (pos1.0 as i32, pos1.1 as i32);
    let pos2 = (pos2.0 as i32, pos2.1 as i32);
    return (pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)) as i32;
}
