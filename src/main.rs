mod code;

#[allow(unused_imports)]
mod lib;

#[allow(unused_imports)]
fn main() {
    use code::*;
    lib::run(day10_1::solve, "day10-sample.txt");
    // lib::run(day10_1::solve, "day9.txt");
}
