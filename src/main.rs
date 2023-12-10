mod code;
#[path = "utils/lib.rs"]
mod lib;

fn main() {
    use code::*;
    lib::log(day7_2::solve, "day7-sample.txt");
    lib::log(day7_2::solve, "day7.txt");
    // day5_2::test();
}
