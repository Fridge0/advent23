use itertools::Itertools;

pub fn solve(input: String) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut entries = split_at_true(lines, |str| str.contains("map:"));
    let seeds = Ranges::from(entries.remove(0)[0]);
    let mut maps = Vec::new();
    for entry in entries {
        let keys = Vec::from(entry);
        let keys = keys.iter().filter_map(|str| parse_key(str)).collect_vec();
        maps.push(Map::from_keys(keys));
    }
    assert_eq!(maps.len(), 7);
    if let [seed_soil, soil_fer, fer_water, water_light, light_temp, temp_humid, humid_loc] =
        &maps[0..7]
    {
        let locations = seeds
            .apply(seed_soil)
            .apply(soil_fer)
            .apply(fer_water)
            .apply(water_light)
            .apply(light_temp)
            .apply(temp_humid)
            .apply(humid_loc);
        locations.get_min()
    } else {
        panic!();
    }
}
#[allow(dead_code)]
pub fn test() {
    test_range();
}
#[derive(Clone, Debug)]
struct Map {
    keys: Vec<Key>,
}
impl Map {
    fn from_keys(keys: Vec<Key>) -> Map {
        Self { keys }
    }
    fn iter(&self) -> std::slice::Iter<Key> {
        self.keys.iter()
    }
}
#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    over: usize,
}
impl Range {
    pub fn from(sl: &[i64]) -> Self {
        Self {
            start: sl[0] as usize,
            over: (sl[0] + sl[1]) as usize,
        }
    }
    pub fn new(start: usize, over: usize) -> Self {
        Self { start, over }
    }
    pub fn extract_src(key: Key) -> Self {
        Self::new(key.src, key.src + key.len)
    }
    fn overlaps(&self, other: &Range) -> bool {
        if self.start >= other.over || self.over <= other.start {
            return false;
        }
        true
    }
    fn and_not(&self, other: Range) -> (Option<Range>, Option<Range>) {
        let mut count = 0;
        if self.start < other.start {
            count += 1;
        }
        if self.over > other.over {
            count += 1;
        }
        if count == 0 {
            return (None, None);
        } else if count == 1 {
            if self.start < other.start {
                return (
                    Some(Range {
                        start: self.start,
                        over: other.start,
                    }),
                    None,
                );
            } else {
                return (
                    Some(Range {
                        start: other.over,
                        over: self.over,
                    }),
                    None,
                );
            }
        } else {
            return (
                Some(Range {
                    start: self.start,
                    over: other.start,
                }),
                Some(Range {
                    start: other.over,
                    over: self.over,
                }),
            );
        }
    }
    fn subtract(&self, other: Range) -> (Option<Range>, Vec<Range>) {
        if !self.overlaps(&other) {
            return (None, vec![self.clone()]);
        }
        let mut count = 0;
        if self.start < other.start {
            count += 1;
        }
        if self.over > other.over {
            count += 1;
        }
        if count == 0 {
            // ( [ ] ) | [] == self : pattern A
            return (Some(*self), vec![]);
        }
        if count == 1 {
            if self.start < other.start {
                // [  (  ]  ) [] = self : pattern B
                return (
                    Some(Range::new(other.start, self.over)),
                    vec![Range::new(self.start, other.start)],
                );
            } else {
                // ( [ ) ] | [] = self : pattern C
                return (
                    Some(Range::new(self.start, other.over)),
                    vec![Range::new(other.over, self.over)],
                );
            }
        }
        // count == 2
        // ( [ ] ) | self = [] : pattern D
        return (
            Some(Range::new(other.start, other.over)),
            vec![
                Range::new(self.start, other.start),
                Range::new(other.over, self.over),
            ],
        );
    }
    fn concat(&self, other: Range) -> Self {
        Self {
            start: std::cmp::min(self.start, other.start),
            over: std::cmp::max(self.over, other.over),
        }
    }
    fn shift_by(self, shift_len: i64) -> Self {
        if self.start as i64 + shift_len < 0 {
            panic!("overrun");
        }
        let start = self.start as i64 + shift_len;
        let over = self.over as i64 + shift_len;
        Self::new(start as usize, over as usize)
    }
    fn validate(&self) -> Option<Self> {
        if self.start >= self.over {
            return None;
        }
        return Some(*self);
    }
}
#[allow(dead_code)]
fn test_range() {
    let one_five = Range::new(1, 5);
    let zero_six = Range::new(0, 6);
    let two_eight = Range::new(2, 8);
    println!("{:?}", one_five.subtract(zero_six)); // should get 1..5 and nothing remain (pattern A)
    println!("{:?}", zero_six.subtract(two_eight)); // should get 2..6 and remain 0..2 (pattern B)
    println!("{:?}", two_eight.subtract(zero_six)); // should get 2..6 and remain 6..8 (pattern C)
    println!("{:?}", zero_six.subtract(one_five)); // should get 1..5 and 0..1 and 5..6 as remaining (pattern D)
}
#[derive(Debug)]
struct Ranges(Vec<Range>);
impl Ranges {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push(&mut self, r: Range) {
        self.0.push(r);
    }
    fn iter(&self) -> std::slice::Iter<Range> {
        self.0.iter()
    }
    fn from(str: &str) -> Self {
        let mut new = Self::new();
        let sets = str
            .split(" ")
            .into_iter()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect_vec();
        for slice in sets.chunks_exact(2) {
            new.push(Range::from(slice));
        }
        new
    }
    fn apply(self, map: &Map) -> Self {
        self.get_min();
        let new = self
            .iter()
            .map(|range| map_range(range, &map.keys))
            .collect::<Vec<_>>()
            .concat();
        Self(new).simplify()
    }
    fn get_min(&self) -> usize {
        let min = self.0.iter().map(|range| range.start).min().unwrap_or(0);
        println!("current min: {}", min);
        min
    }
    fn simplify(self) -> Self {
        if self.0.len() == 0 {
            return self;
        }
        let mut new = Vec::new();
        for &range in self.iter() {
            if new.len() == 0 {
                new.push(range);
            } else {
                let last = new.pop().unwrap();
                if last.overlaps(&range) {
                    new.push(last.concat(range));
                } else {
                    new.push(last);
                    new.push(range);
                }
            }
        }
        let new = new.iter().filter_map(|x| x.validate()).collect_vec();
        Self(new)
    }
}
fn map_range(range: &Range, keys: &Vec<Key>) -> Vec<Range> {
    let mut retval = vec![*range];
    for key in keys {
        if range.overlaps(&Range::extract_src(*key)) {
            let result = range.subtract(Range::extract_src(*key));
            retval = vec![vec![result.0.unwrap().shift_by(key.diff())], result.1].concat();
            break;
        }
    }
    return retval;
}
fn parse_key(str: &str) -> Option<Key> {
    let atoms = str.split(" ").collect_vec();
    Some(Key {
        dest: atoms[0].parse().ok()?,
        src: atoms[1].parse().ok()?,
        len: atoms[2].parse().ok()?,
    })
}
#[derive(Clone, Copy, Debug)]
struct Key {
    dest: usize,
    src: usize,
    len: usize,
}
impl Key {
    fn dest_range(&self) -> Range {
        Range {
            start: self.dest,
            over: self.dest + self.len,
        }
    }
    fn src_range(&self) -> Range {
        Range {
            start: self.src,
            over: self.src + self.len,
        }
    }
    fn diff(&self) -> i64 {
        self.dest as i64 - self.src as i64
    }
}

fn split_at_true<T: Clone>(vec: Vec<T>, func: impl Fn(&T) -> bool) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut slice = Vec::new();
    for elem in vec.iter() {
        if func(elem) {
            // split
            result.push(slice);
            slice = Vec::new();
        } else {
            slice.push(elem.clone());
        }
    }
    if !slice.is_empty() {
        result.push(slice);
    }
    return result;
}

fn into_seeds(str: &str) -> Vec<usize> {
    str.split(" ")
        .filter_map(|item| item.parse().ok())
        .collect_vec()
}
fn larger(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}
