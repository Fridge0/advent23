#[allow(dead_code)]
pub mod field {
    use std::{clone, fmt::Formatter};

    use itertools::Itertools;

    pub struct Vector(i32, i32);
    #[derive(Clone, Debug, PartialEq)]
    pub struct Pos(pub usize, pub usize);
    #[derive(Clone, PartialEq)]
    pub struct Field<T> {
        field: Vec<Vec<T>>,
    }
    impl<T: Clone> Field<T> {
        pub fn new() -> Self {
            let new: Vec<Vec<T>> = Vec::new();
            return Self { field: new };
        }
        pub fn from(v: Vec<Vec<T>>) -> Self {
            return Self { field: v };
        }
        pub fn len(&self) -> usize {
            return self.field.iter().map(|vec| vec.len()).sum();
        }
        pub fn height(&self) -> usize {
            return self.field.len();
        }
        pub fn width(&self) -> usize {
            if self.field.len() == 0 {
                return 0; // we don't really know how long it would be, but it be better considered 0 rather than unknown
            } else {
                self.field[0].len()
            }
        }
        pub fn rows(&self) -> Vec<Vec<T>> {
            return self.field.to_owned();
        }
        pub fn cols(&self) -> Vec<Vec<T>> {
            if self.width() == 0 {
                Vec::new()
            } else {
                let mut cols = vec![Vec::new(); self.width()];
                for row in self.field.iter() {
                    for (idx, item) in row.iter().enumerate() {
                        cols[idx].push(item.to_owned());
                    }
                }
                cols
            }
        }
        pub fn pushln(&mut self, line: Vec<T>) {
            self.field.push(line);
        }
        pub fn push_row(&mut self, line: Vec<T>) {
            self.field.push(line);
        }
        pub fn push_col(&mut self, col: Vec<T>) {
            let mut col = col.iter();
            for ln in self.field.iter_mut() {
                ln.push(
                    col.next()
                        .expect("not enough len of input | happened in push col")
                        .to_owned(),
                );
            }
        }
        pub fn push_row_fill(&mut self, default: T) {
            self.push_row(vec![default; self.width()]);
        }
        pub fn push_col_fill(&mut self, default: T) {
            self.push_col(vec![default; self.height()]);
        }
        pub fn iter_vec(&self) -> std::slice::Iter<Vec<T>> {
            self.field.iter().to_owned()
        }
        pub fn with_size(x: usize, y: usize, default: T) -> Self {
            let mut new = Self::new();
            for _ in 0..x {
                new.pushln(vec![default.clone(); y]);
            }
            new
        }
        pub fn get(&self, pos: &Pos) -> Option<T> {
            if pos.0 >= self.field.len() {
                return None;
            }
            if pos.1 >= self.field[pos.0].len() {
                return None;
            }
            return Some(self.field[pos.0][pos.1].clone());
        }
        pub fn set(&mut self, pos: &Pos, value: T) {
            if pos.0 >= self.field.len() {
                return;
            }
            if pos.1 >= self.field[pos.0].len() {
                return;
            }
            self.field[pos.0][pos.1] = value;
        }
        pub fn concat(&self) -> Vec<T> {
            return self.field.concat();
        }
        pub fn find_position<F>(&self, func: F) -> Option<Pos>
        where
            F: Fn(&T) -> bool,
        {
            for (x, ln) in self.field.iter().enumerate() {
                for (y, item) in ln.iter().enumerate() {
                    if func(item) {
                        return Some(Pos(x, y));
                    }
                }
            }
            return None;
        }
        pub fn find_all_position<F>(&self, func: F) -> Vec<Pos>
        where
            F: Fn(&T) -> bool,
        {
            let mut result = Vec::new();
            for (x, ln) in self.field.iter().enumerate() {
                for (y, item) in ln.iter().enumerate() {
                    if func(item) {
                        result.push(Pos(x, y));
                    }
                }
            }
            return result;
        }
        pub fn map<U, F>(&self, func: F) -> Field<U>
        where
            F: Fn(T) -> U,
            U: Clone,
        {
            let new: Field<U> = Field::from({
                self.field
                    .clone()
                    .into_iter()
                    .map(|vec| vec.into_iter().map(|elem| func(elem)).collect_vec())
                    .collect::<Vec<Vec<U>>>()
            });
            new
        }
    }

    impl Field<char> {
        pub fn parse_str(str: String) -> Self {
            let field = str
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec();
            Self { field }
        }
    }
    impl<T> std::fmt::Debug for Field<T>
    where
        T: std::fmt::Display,
    {
        fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
            for ln in self.field.iter() {
                write!(
                    fmt,
                    "{}\n",
                    ln.iter().map(|elem| format!("{}", elem)).join("")
                )?;
            }
            return Ok(());
        }
    }
    pub fn test() {
        use super::log;
        let field = Field::with_size(10, 10, 1);
        log(field.map(|x| x * 3));
    }
}

#[allow(dead_code)]
pub fn run<T: std::fmt::Debug>(func: impl Fn(String) -> T, path: &str) {
    let path = "src/txt/".to_owned() + path;
    let input = std::fs::read_to_string(path).unwrap();
    println!("{:?}", func(input));
}
pub fn log<T: std::fmt::Debug>(anything: T) {
    println!("{anything:?}");
}
pub fn read(path: &str) -> String {
    let path = "src/txt/".to_string() + path;
    let result: Result<String, std::io::Error> = std::fs::read_to_string(path.to_string());
    result.expect("ERROR: Path doesn't exist")
}
#[allow(dead_code)]
pub fn dbg(x: impl std::fmt::Debug) {
    println!("{:?}", x);
}
