use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Container that will reset its value to 0 when it goes over a maximum value
struct Index {
    max: usize,
    value: usize
}

/// Generate an iterator of all possible permutations for indexes in a Vec.
/// It isn't the most efficient algorithm, but it's the one that came to mind.
/// Afterwards I checked wikipedia and discovered that there's a much more efficient
/// algorithm for generating permutations, but hey this just for fun !
struct Permutation {
    indexes: Vec<Index>,
}

impl Index {

    fn new (max: usize) -> Index {
        Index {
            max: max,
            value: 0
        }
    }

    /// Increment the value and return true if it has overflowed to 0
    fn increment(&mut self) -> bool {
        self.value += 1;
        if self.value == self.max {
            self.value = 0;
            true
        } else {
            false
        }
    }

}

impl Permutation {

    fn new(size: usize, max: usize) -> Permutation {
        Permutation {
            indexes: (0..size).map(|_| Index::new(max)).collect()
        }
    }

    /// Increment all indexes in the permutation. Return true if the last index has overflowed back to 0.
    /// when the last index has overflowed, then we have iterated through all possible permutations.
    fn increment(&mut self) -> bool {
        let mut index = 0;
        loop {
            if !self.indexes[index].increment() {
                return false;
            }

            if index == self.indexes.len() - 1 {
                return true;
            }
            index += 1;
        }
    }

    /// Used to make sure we don't accidentally combine the same index more than once
    fn has_duplicates(&self) -> bool {
        for i in 0..self.indexes.len() {
            for j in 0..self.indexes.len() {
                if i != j && self.indexes[i].value == self.indexes[j].value {
                    return true;
                }
            }
        }
        false
    }
}

impl Iterator for Permutation {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        loop {
            if self.increment() {
                return None;
            }
            if !self.has_duplicates() {
                let values = self.indexes.iter().map(|i| i.value).collect();
                return Some(values);
            }
        }
    }

}

fn main() {
    let permutations: usize = env::args()
        .nth(1).expect("number of permutations missing")
        .parse().expect("error parsing number of permutations");

    let path = env::args()
        .nth(2).expect("path to file missing");

    let file = File::open(path).expect("cannot open file");
    let reader = BufReader::new(file);

    let numbers: Vec<u32> = reader.lines()
        .map(|result| result
            .expect("error reading line")
            .parse()
            .expect("error parsing number")
        )
        .collect();

    let permutations = Permutation::new(permutations, numbers.len());
    
    let total: u32 = permutations
        .map(|indexes| {
            indexes.iter().map(|i| numbers[*i]).collect::<Vec<u32>>()
        })
        .filter(|selected| {
            selected.iter().sum::<u32>() == 2020
        })
        .map(|selected| selected.iter().product())
        .next()
        .expect("no numbers found that sum to 2020");

    println!("{}", total);
}