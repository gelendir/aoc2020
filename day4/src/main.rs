use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::ops::RangeInclusive;

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const DIGITS: RangeInclusive<char> = '0'..='9';
const LETTERS: RangeInclusive<char> = 'a'..='f';
const HEIGHT_CM: RangeInclusive<u8> = 150..=193;
const HEIGHT_IN: RangeInclusive<u8> = 59..=76;


#[derive(Debug)]
struct Passport(HashMap<String, String>);


impl Passport {

    fn new() -> Passport {
        Passport(HashMap::new())
    }

    fn read(path: &str) -> Vec<Passport> {
        let file = File::open(path).expect("cannot open file");
        let buffer = BufReader::new(file);
        let mut passports = vec![Passport::new()];

        for result in buffer.lines() {
            let line = result.expect("cannot read line");
            match line.as_str() {
                "" => passports.push(Passport::new()),
                text => passports.last_mut().unwrap().parse(text)
            }
        }

        passports
    }

    fn parse(&mut self, text: &str) {
        text.split(" ")
            .for_each(|token| {
                let mut parts = token.split(":");
                let key = parts.next().expect("key not found").to_string();
                let value = parts.next().expect("value not found").to_string();
                self.0.insert(key, value);
            })
    }

    fn is_valid(&self) -> bool {
        let valid = self.0.iter()
            .filter(|(key, value)| match key.as_str() {
                    "byr" => Self::validate_range(value, 1920, 2002),
                    "iyr" => Self::validate_range(value, 2010, 2020),
                    "eyr" => Self::validate_range(value, 2020, 2030),
                    "hgt" => Self::validate_height(value),
                    "hcl" => Self::validate_hair_color(value),
                    "ecl" => Self::validate_eye_color(value),
                    "pid" => Self::validate_passport_id(value),
                    "cid" => true,
                    _ => false
            })
            .count();

        match valid {
            8 => true,
            7 => !self.0.contains_key("cid"),
            _ => false
        }
    }

    fn validate_range(value: &str, min: u16, max: u16) -> bool {
        value.parse::<u16>()
            .map(|v| v >= min && v <= max)
            .unwrap_or(false)
    }

    fn validate_height(value: &str) -> bool {
        if value.len() < 3 {
            return false;
        }

        let split = value.len() - 2;
        let number = &value[0..split];
        let unit = &value[split..];

        number.parse::<u8>()
            .map(|n| match unit {
                "cm" => HEIGHT_CM.contains(&n),
                "in" => HEIGHT_IN.contains(&n),
                _ => false
            })
            .unwrap_or(false)
    }

    fn validate_hair_color(value: &str) -> bool {
        if value.len() != 7 {
            return false;
        }

        if &value[0..1] != "#" {
            return false;
        }

        value[1..].chars()
            .all(|c| DIGITS.contains(&c) || LETTERS.contains(&c))
    }

    fn validate_eye_color(value: &str) -> bool {
        EYE_COLORS.contains(&value)
    }

    fn validate_passport_id(value: &str) -> bool {
        value.chars()
            .filter(|c| DIGITS.contains(&c))
            .count() == 9
    }

}

fn main() {
    let path = env::args().nth(1).expect("missing file path");
    let passports = Passport::read(&path);

    let valid = passports.iter()
        .filter(|p| p.is_valid())
        .count();

    println!("{}", valid);
}
