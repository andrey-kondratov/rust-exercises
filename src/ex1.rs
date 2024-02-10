use std::collections::HashMap;

#[derive(Debug)]
pub struct Result {
    pub median: i32,
    pub mode: i32,
}

pub fn main(input: &Vec<i32>) -> Result {
    let mut sorted = input.to_owned();
    sorted.sort();
    let median_index = sorted.len() / 2 + 1;
    let median = sorted[median_index];

    let mut counters = HashMap::new();
    let mut max_count = 0;
    let mut mode: i32 = 0;
    for integer in sorted {
        let count = counters.entry(integer).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count += 1;
            mode = integer;
        }
    }

    Result { median, mode }
}
