use crate::lib;
use lib::debug;
use lib::parse;
use lib::Field2d;

pub fn solve(str: String) -> i32 {
    let field = Field2d::from_string(str);
    let numbers = field
        .lines()
        .map(|str| str.to_owned())
        .map(parse::numbers_enum);
    let mut sum = 0;
    for (j, line) in numbers.enumerate() {
        for ((start, over), number) in line {
            if (start..over).any(|y| look_around(&field, (j as i32, y))) {
                sum += number;
            };
        }
    }
    return sum;
}

pub fn test() {
    let f = Field2d::from_string(lib::read_file("src/txt/day3-sample.txt"));
    debug(look_around(&f, (2, 0)));
    debug(look_at(&f, (3, 1)));
}

fn look_around(field: &Field2d<char>, (x, y): (i32, i32)) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            if look_at(field, (i + x, j + y)) {
                return true;
            }
        }
    }
    return false;
}
fn look_at(field: &Field2d<char>, (x, y): (i32, i32)) -> bool {
    match parse::char(field.get(x, y).unwrap_or('.')) {
        Ok(_some) => return false,
        Err(char) => {
            if char == '.' {
                return false;
            } else {
                return true;
            }
        }
    }
}
fn look_at_point(point: char) -> bool {
    match parse::char(point) {
        Ok(_some) => return false,
        Err(char) => {
            if char == '.' {
                return false;
            } else {
                return true;
            }
        }
    }
}
