use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;


fn parse(path: &str) -> Vec<u16> {
    let file = File::open(path).expect("cannot open file");
    let buffer = BufReader::new(file);
    buffer.lines()
        .map(|result| result.expect("cannot read line"))
        .map(|line| line.parse().expect("cannot parse line"))
        .collect()
}

fn calculate_jolt_differences(joltages: &Vec<u16>) -> u16 {
    let after = &joltages[1..];

    let mut deltas: HashMap<u16, u16> = HashMap::new();
    joltages.iter()
        .zip(after)
        .map(|(low, high)| high - low)
        .for_each(|delta| {
            let counter = deltas.entry(delta).or_insert(0);
            *counter += 1;
        });

    deltas.values().product()
}

fn main() {
    let path = env::args().nth(1).expect("no path to file");

    let mut joltages = parse(&path);
    joltages.sort();

    let device_joltage = joltages.last()
        .map(|j| j + 3)
        .expect("no highest joltage");

    joltages.insert(0, 0);
    joltages.push(device_joltage);

    let differences = calculate_jolt_differences(&joltages);

    println!("differences: {}", differences);
}
