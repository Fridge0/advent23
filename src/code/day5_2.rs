use itertools::Itertools;

pub fn solve(input: String) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut entries = split_at_true(lines, |str| str.contains("map:"));
    let seeds = into_seeds(entries.remove(0)[0]);
    let mut maps = Vec::new();
    for entry in entries {
        let keys = Vec::from(entry);
        let keys = keys.iter().filter_map(|str| parse_key(str)).collect_vec();
        maps.push(Map::from_keys(keys));
    }
    let (seed_soil, soil_fer, fer_water, water_light, light_temp, temp_humid, humid_loc) = (
        maps[0].clone(),
        maps[1].clone(),
        maps[2].clone(),
        maps[3].clone(),
        maps[4].clone(),
        maps[5].clone(),
        maps[6].clone(),
    );

    let locations = seeds
        .into_iter()
        .map(|seed| seed_soil.apply(seed))
        .map(|soil| soil_fer.apply(soil))
        .map(|fer| fer_water.apply(fer))
        .map(|water| water_light.apply(water))
        .map(|light| light_temp.apply(light))
        .map(|temp| temp_humid.apply(temp))
        .map(|humid| humid_loc.apply(humid));
    locations.min().unwrap()
}

#[derive(Clone)]
struct Map {
    keys: Vec<Key>,
}
impl Map {
    fn from_keys(keys: Vec<Key>) -> Map {
        Self { keys }
    }
    fn apply(&self, i: usize) -> usize {
        let target_key = self
            .keys
            .iter()
            .find(|key| key.src <= i && i < key.src + key.len);
        if let Some(target_key) = target_key {
            return target_key.dest + i - target_key.src;
        } else {
            return i;
        }
    }
}

fn parse_key(str: &str) -> Option<Key> {
    let atoms = str.split(" ").collect_vec();
    Some(Key {
        // we can assure that the input is correct, tf. not care about unwrapping them
        dest: atoms[0].parse().ok()?,
        src: atoms[1].parse().ok()?,
        len: atoms[2].parse().ok()?,
    })
}
#[derive(Clone)]
struct Key {
    dest: usize,
    src: usize,
    len: usize,
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
