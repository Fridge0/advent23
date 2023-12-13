mod code;

#[allow(unused_imports, special_module_name)]
mod lib;

#[allow(unused_imports)]
fn main() {
    use code::*;
    lib::run(day10_2::solve, "day10-sample.txt");
    lib::run(day10_2::solve, "day10-sample-2.txt");
    lib::run(day10_2::solve, "day10-sample-3.txt");
    lib::run(day10_2::solve, "day10.txt");
    // test();
}
