mod code;

#[allow(unused_imports, special_module_name)]
mod lib;

#[allow(unused_imports)]
fn main() {
    use code::*;
    lib::run(day11_1::solve, "day11-sample.txt");
    lib::run(day11_1::solve, "day11.txt");
    // test();
}
