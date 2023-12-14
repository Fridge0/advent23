mod code;

#[allow(unused_imports)]
mod utils;

#[allow(unused_imports)]
use code::*;

fn main() {
    // utils::run(day11_1::solve, "day11-test.txt");
    let input = utils::read("day11.txt");
    utils::log(day11_2::solve(input, 1000000));
    // utils::run(day11_1::solve, "day11.txt");
    // test();
}
fn _main() {
    // _main();
    test();
}
#[allow(dead_code)]
fn test() {
    let input = utils::read("day11-sample.txt");
    utils::log(day11_2::solve(input, 2));
    // expect 374
}
