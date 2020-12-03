use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Entry {
    min: usize,
    max: usize,
    letter: char,
    password: String
}

enum Mode {
    Count,
    Position
}

impl Mode {

    fn is_valid(&self, entry: &Entry) -> bool {
        match self {
            Mode::Count => Self::validate_count(entry),
            Mode::Position => Self::validate_position(entry),
        }
    }

    fn validate_count(entry: &Entry) -> bool {
        let count = entry.password.chars()
        .filter(|c| c == &entry.letter)
        .count();
        count >= entry.min && count <= entry.max
    }

    fn validate_position(entry: &Entry) -> bool {
        let count = [entry.min, entry.max]
            .iter()
            .filter(|pos| {
                let letter = entry.password.chars().nth(*pos - 1).expect("cannot find letter");
                letter == entry.letter
            })
            .count();
        count == 1
    }
}

impl Entry {

    fn parse(line: String) -> Entry {
        let mut parts = line.split(" ");

        let range = parts.next().expect("range not found");

        let letter = parts.next()
            .expect("letter not found")
            .trim_end_matches(":")
            .parse()
            .expect("cannot parse letter");

        let password = parts.next().expect("password not found");

        let mut range = range.split("-");
        let min: usize = range.next()
            .expect("min not found")
            .parse()
            .expect("cannot parse min");
        let max: usize = range.next()
            .expect("max not found")
            .parse()
            .expect("cannot parse max");

        Entry {
            min: min,
            max: max,
            letter: letter,
            password: password.to_string()
        }
    }

}

fn main() {
    let mode = env::args().nth(1).expect("missing mode");
    let mode = match mode.as_str() {
        "count" => Mode::Count,
        "position" => Mode::Position,
        _ => panic!("invalid mode")
    };

    let path = env::args().nth(2).expect("missing path to file");
    let file = File::open(path).expect("cannot open file");
    let buffer = BufReader::new(file);

    let entries: Vec<Entry> = buffer.lines()
        .map(|l| Entry::parse(l.expect("cannot read line")))
        .collect();

    let valid = entries.iter()
        .filter(|e| mode.is_valid(&e))
        .count();
        
    println!("{}", valid)
}
