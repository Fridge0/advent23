mod code;
#[path = "utils/lib.rs"]
mod lib;

fn main() {
    use code::*;
    lib::log(day5_2::solve, "day5-sample.txt");
    lib::log(day5_2::solve, "day5.txt");
    // day5_2::test();
}
