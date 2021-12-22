use std::cmp::{Eq, Ord, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Clone)]
pub struct Reading {
    pub value: Vec<u8>
}

pub struct LifeSupport {
    pub oxygen_generator_rating: u32,
    pub co2_scrubber_rating: u32,
    pub value: u32
}

impl Reading {

    pub fn from_str(line: String) -> Reading {
        Reading{value: line
            .chars()
            .map(map_boolean)
            .collect::<Vec<u8>>()}
    }

    pub fn oxygen_filter(readings: Vec<Reading>, index: u8) -> &'static dyn Fn(Reading) -> bool {
        let most_common = most_frequent(readings
            .into_iter()
            .map(|reading: Reading| reading.value)
            .map(|reading| reading[index as usize])
            .collect::<Vec<u8>>());
        &move |reading: Reading| reading.value[index as usize] == most_common
    }

    pub fn co2_filter(readings: Vec<Reading>, index:u8) -> &'static dyn Fn(Reading) -> bool {
        let least_common = most_frequent(readings
            .into_iter()
            .map(|reading: Reading| reading.value)
            .map(|reading| reading[index as usize])
            .collect::<Vec<u8>>());
        &move |reading: Reading| reading.value[index as usize] == least_common
    }

    pub fn get_value(&self) -> u32 {
        self.value.into_iter().fold(0, |acc, b| acc * 2 + b) as u32
    }
}

fn map_boolean(c: char) -> u8 {
    match c {
        '1' => 1,
        '0' => 0,
        _ => panic!("Input contains non-0 non-1 characters")
    }
}

pub fn most_frequent<T>(array: Vec<T>) -> T
where
    T: Hash + Eq + Ord,
{
    let mut map: HashMap<T, u8> = HashMap::new();
    for x in array {
        *map.entry(x).or_default() += 1;
    }

    let mut heap = BinaryHeap::with_capacity(2);
    for (x, count) in map.into_iter() {
        heap.push(Reverse((count, x)));
        if heap.len() > 1 {
            heap.pop();
        }
    }
    heap.peek().unwrap().0.1
}
