use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse(path: &str) -> Vec<u64> {
    let file = File::open(path).expect("cannot open file");
    let buffer = BufReader::new(file);
    buffer.lines()
        .map(|result| result.expect("cannot read line"))
        .map(|line| line.parse().expect("cannot parse line"))
        .collect()
}

fn find_invalid(preamble: usize, numbers: &Vec<u64>) -> u64 {
    let invalid = numbers[preamble..].iter()
        .enumerate()
        .filter_map(|(start, number)| {
            let end = start + preamble;
            match is_valid(*number, &numbers[start..end]) {
                true => None,
                false => Some(number)
            }  
        })
        .next()
        .expect("no invalid numbers found");
    *invalid
}

fn is_valid(needle: u64, numbers: &[u64]) -> bool {
    numbers.iter()
        .enumerate()
        .any(|(i, outer)| {
            numbers.iter()
                .enumerate()
                .any(|(j, inner)| {
                    i != j && outer + inner == needle
                })
        })
}

fn find_sum(invalid: u64, numbers: &Vec<u64>) -> Vec<u64> {
    (0..numbers.len())
        .filter_map(|start| {
            let mut sum = 0;
            let end = (start..numbers.len())
                .filter(|j| {
                    sum += numbers[*j];
                    sum >= invalid
                })
                .next();

            if sum == invalid {
                let end = end.unwrap();
                let sum = numbers[start..end].iter().cloned().collect();
                Some(sum)
            } else {
                None
            }
        })
        .next()
        .expect("no sum of numbers found")
}

fn main() {
    let preamble: usize = env::args().nth(1)
        .expect("preamble not found")
        .parse()
        .expect("cannot parse preamble");

    let path = env::args().nth(2).expect("path not found");

    let numbers = parse(&path);
    let invalid = find_invalid(preamble, &numbers);
    println!("invalid: {}", invalid);

    let mut sum = find_sum(invalid, &numbers);
    sum.sort();

    println!("weakness: {}", sum[0] + sum[sum.len()-1])
}
