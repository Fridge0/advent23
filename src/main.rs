mod code;
// #[path = "utils/lib.rs"]
// mod lib;

fn main() {
    use code::*;
    log(day8_2::solve, "day8-sample-2.txt");
    log(day8_2::solve, "day8.txt");
    // day5_2::test();
}

fn log<T: std::fmt::Debug>(func: impl Fn(String) -> T, path: &str) {
    let path = "src/txt/".to_owned() + path;
    let input = std::fs::read_to_string(path).unwrap();
    println!("{:?}", func(input));
}
