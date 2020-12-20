use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Bit {
    Zero,
    One,
    X
}

#[derive(Debug)]
struct BitMask (Vec<Bit>);

#[derive(Debug)]
struct Assignment {
    address: u64,
    value: u64
}

impl BitMask {

    fn empty() -> BitMask {
        BitMask(Vec::new())
    }

    fn parse(text: &str) -> Option<BitMask> {
        let regex = Regex::new(r"mask = ([01X]+)").expect("cannot parse regex");

        if let Some(captures) = regex.captures(&text) {
            let mask = captures[1].chars()
                .map(|c| match c{
                    '0' => Bit::Zero,
                    '1' => Bit::One,
                    'X' => Bit::X,
                    _ => panic!("unexpected bit char")
                })
                .collect();
            Some(BitMask(mask))
        } else {
            None
        }
    }

    fn apply(&self, value: u64) -> u64 {
        self.0.iter().rev()
            .enumerate()
            .fold(value, |base, (i, bit)| match bit {
                Bit::One => base | 1 << i,
                Bit::Zero => base & !(1 << i),
                Bit::X => base
            })
    }

    fn addresses(&self, address: u64) -> Vec<u64> {
        let address = self.0.iter()
            .rev()
            .enumerate()
            .fold(address, |base, (i, bit)| {
                let value = match bit {
                    Bit::One => base | 1 << i,
                    _ => base
                };
                value
            });

        let floats: Vec<usize> = self.0.iter()
            .rev()
            .enumerate()
            .filter_map(|(i, bit)| match bit {
                Bit::X => Some(i),
                _ => None
            })
            .collect();

        let max = (2 as u64).pow(floats.len() as u32);

        (0..=max)
            .map(|i| {
                floats.iter()
                    .zip(Self::bit_vec(i, floats.len()).iter())
                    .fold(address, |base, (i, b)| {
                        let value = match b {
                            true => base | 1 << i,
                            false => base & !(1 << i)
                        };
                        value
                    })
            })
            .collect()        
    }

    fn bit_vec(value: u64, size: usize) -> Vec<bool> {
        (0..size)
            .map(|i| value & 1<<i != 0)
            .rev()
            .collect()
    }

}

impl Assignment {

    fn parse(text: &str) -> Option<Assignment> {
        let regex = Regex::new(r"mem\[(\d+)\] = (\d+)")
            .expect("cannot parse regex");

        if let Some(captures) = regex.captures(&text) {
            let address = captures[1].parse().expect("address is not integer");
            let value = captures[2].parse().expect("value is not integer");
            Some(Assignment{
                address: address,
                value: value
            })
        } else {
            None
        }
    }

    fn execute_value(&self, memory: &mut HashMap<u64, u64>, bitmask: &BitMask) {
        memory.insert(
            self.address,
            bitmask.apply(self.value)
        );
    }

    fn execute_address(&self, memory: &mut HashMap<u64, u64>, bitmask: &BitMask) {
        bitmask.addresses(self.address)
            .iter()
            .for_each(|address| {
                memory.insert(*address, self.value);
            });
    }
        
}


fn main() {
    let mode = env::args().nth(1).expect("no mode");
    let path = env::args().nth(2).expect("file path missing");
    let file = File::open(path).expect("cannot open file");
    let buffer = BufReader::new(file);

    let mut bitmask = BitMask::empty();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    buffer.lines()
        .map(|result| result.expect("cannot read line"))
        .for_each(|line| {
            if let Some(mask) = BitMask::parse(&line) {
                bitmask = mask;
            } else if let Some(assignment) = Assignment::parse(&line) {
                if mode == "value" {
                    assignment.execute_value(&mut memory, &bitmask);
                } else {
                    assignment.execute_address(&mut memory, &bitmask);
                }
            } else {
                panic!("line cannot be parsed");
            }
        });

    let sum: u64 = memory.values()
        .filter(|v| **v != 0)
        .sum();

    println!("{:?}", memory);
    println!("{}", sum);
}
