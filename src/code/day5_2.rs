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
    // let (seed_soil, soil_fer, fer_water, water_light, light_temp, temp_humid, humid_loc) = (
    //     maps[0].clone(),
    //     maps[1].clone(),
    //     maps[2].clone(),
    //     maps[3].clone(),
    //     maps[4].clone(),
    //     maps[5].clone(),
    //     maps[6].clone(),
    // );
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

#[derive(Clone)]
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
    pub fn from(sl: &[i32]) -> Self {
        Self {
            start: sl[0] as usize,
            over: (sl[0] + sl[1]) as usize,
        }
    }
    pub fn new(start: usize, over: usize) -> Self {
        Self { start, over }
    }
    fn overlaps(&self, other: &Range) -> bool {
        if self.start >= other.over || self.over <= other.start {
            return false;
        }
        true
    }
    fn and(&self, other: &Range) -> Range {
        Range {
            start: if self.start > other.start {
                self.start
            } else {
                other.start
            },
            over: if self.over < other.over {
                self.over
            } else {
                other.over
            },
        }
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
            // ( [ ] ) | [] == self
            return (Some(*self), vec![]);
        }
        if count == 1 {
            if self.start < other.start {
                // [  (  ]  ) [] = self
                return (
                    Some(Range::new(other.start, self.over)),
                    vec![Range::new(self.start, other.start)],
                );
            } else {
                // ( [ ) ] | [] = self
                return (
                    Some(Range::new(self.start, other.over)),
                    vec![Range::new(other.over, self.over)],
                );
            }
        }
        // count == 2
        // ( [ ] ) | self = []
        return (
            Some(Range::new(self.start, self.over)),
            vec![
                Range::new(other.start, self.start),
                Range::new(self.over, other.over),
            ],
        );
    }
}
struct Ranges(Vec<Range>);
impl Ranges {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, r: Range) {
        self.0.push(r);
    }
    fn iter(&self) -> std::slice::Iter<Range> {
        self.0.iter()
    }
    pub fn from(str: &str) -> Self {
        let mut new = Self::new();
        let sets = str
            .split(" ")
            .into_iter()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        for slice in sets.chunks_exact(2) {
            new.push(Range::from(slice));
        }
        new
    }
    pub fn apply(mut self, map: &Map) -> Self {
        for range in self.iter() {
            for entry in map.iter() {
                if range.overlaps(&entry.src_range()) {
                    // split range and move some
                }
            }
        }
        self
    }
    pub fn get_min(&self) -> usize {
        todo!()
    }
}

fn parse_key(str: &str) -> Option<Key> {
    let atoms = str.split(" ").collect_vec();
    Some(Key {
        dest: atoms[0].parse().ok()?,
        src: atoms[1].parse().ok()?,
        len: atoms[2].parse().ok()?,
    })
}
#[derive(Clone, Copy)]
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
pub fn test() {
    let one_five = Range::new(1, 6);
    let zero_six = Range::new(0, 7);
    let two_eight = Range::new(2, 8);
    println!("{:?}", zero_six.subtract(two_eight)); // should get 2..7 and remain 0..2
    println!("{:?}", zero_six.subtract(one_five)); // should get 1..6 and 6..7 as remaining
    println!("{:?}", one_five.subtract(zero_six)); // should get 0..6 and nothing remain
}
