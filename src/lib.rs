#[allow(dead_code)]
pub mod field {
    use std::fmt::Formatter;

    use itertools::Itertools;

    pub struct Vector(i32, i32);
    #[derive(Clone, Debug)]
    pub struct Pos(pub usize, pub usize);
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
        pub fn pushln(&mut self, line: Vec<T>) {
            self.field.push(line);
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
