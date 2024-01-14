use crate::lib;
use lib::debug;
use lib::parse;
use lib::Field2d;

pub fn solve(str: String) -> i32 {
    let mut result = 0;
    let f = Field2d::from_string(str.clone());
    let mut vec = Vec::new();
    for (x, line) in str.lines().enumerate() {
        let mut vec_temp = Vec::new();
        for (y, char) in line.chars().enumerate() {
            vec_temp.push(((x, y), char));
        }
        vec.push(vec_temp);
    }
    let f_enum: Field2d<((usize, usize), char)> = Field2d::from(vec);
    let asterisks = f_enum.find_all(|((_, _), char)| *char == '*');
    for ln in asterisks.iter() {
        for ((x, y), _) in ln.iter() {
            let v = look_for_numbers(&f, (x.to_owned() as i32, y.to_owned() as i32));
            if v.len() >= 2 {
                println!("{:?}", v);
            }
            if v.len() == 2 {
                // if asterisk is valid
                result += v.iter().fold(1, |x, y| x * y);
            }
        }
    }
    return result;
}

#[allow(dead_code)]
pub fn test() {
    let f = Field2d::from_string(lib::read_file("src/txt/day3-sample.txt"));
    debug(look_for_numbers(&f, (0, 2)));
    debug(look_at(&f, (3, 1)));
}

fn look_for_numbers(field: &Field2d<char>, (x, y): (i32, i32)) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut adjacent = false;
    for i in -1..=1 {
        for j in -1..=1 {
            if look_at(field, (i + x, j + y)) {
                if !adjacent {
                    stack.push((i + x, j + y));
                }
                adjacent = true;
            } else {
                adjacent = false;
            }
        }
        adjacent = false;
    }
    let mut population = Vec::new();
    for line in field.lines() {
        population.push(parse::numbers_enum(line.to_string()));
    }
    let mut result = Vec::new();
    for coord in stack {
        result.push(
            population[coord.0 as usize]
                .iter()
                .find(|(range, _)| range.0 <= coord.1 && coord.1 < range.1)
                .unwrap()
                .1,
        )
    }
    return result;
}
fn look_at(field: &Field2d<char>, (x, y): (i32, i32)) -> bool {
    match parse::char(field.get(x, y).unwrap_or('.')) {
        Ok(_some) => return true,
        Err(_) => {
            return false;
        }
    }
}
