use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{HashSet, HashMap};
use regex::Regex;

#[derive(Debug)]
struct Bag {
    color: String,
    contents: Vec<BagSet>
}

#[derive(Debug)]
struct BagSet {
    color: String,
    quantity: u32,
}

#[derive(Debug)]
struct ReverseMap(HashMap<String, Vec<String>>);


impl Bag {

    fn read(path: &str) -> Vec<Bag> {
        let file = File::open(path).expect("cannot open file");
        let buffer = BufReader::new(file);
        buffer.lines()
            .map(|result| result.expect("cannot read line"))
            .map(Self::parse)
            .collect()
    }

    fn parse(text: String) -> Bag {
        let mut parts = text.split(" bags contain ");

        let color = parts.next()
            .expect("no color in rule")
            .to_string();

        let bagset = parts.next()
            .expect("no bags in line");

        let regex = Regex::new(r"(no other|(?P<quantity>\d+) (?P<color>[a-z ]+)) bags?(, )?")
            .expect("cannot parse regex");

        let bags: Vec<BagSet> = regex.captures_iter(&bagset)
            .filter_map(|capture| {
                let color = capture.name("color");
                let quantity = capture.name("quantity");
                match (color, quantity) {
                    (Some(c), Some(q)) => Some(BagSet{
                        color: c.as_str().to_string(),
                        quantity: q.as_str().parse().expect("quantity is not a number")
                    }),
                    _ => None
                }
            })
            .collect();

        Bag {
            color: color,
            contents: bags
        }
    }

    fn unpack(&self, bags: &Vec<Bag>) -> u32 {
        self.contents.iter()
            .map(|bagset| bagset.unpack(bags))
            .sum()
    }
}

impl BagSet {

    fn unpack(&self, bags: &Vec<Bag>) -> u32 {
        let bag = bags.iter()
            .find(|bag| bag.color == self.color)
            .expect("bag color not found");

        if bag.contents.is_empty() {
            self.quantity
        } else {
            self.quantity + (self.quantity * bag.unpack(bags))
        }
    }

}

impl ReverseMap {

    fn from_bags(bags: &Vec<Bag>) -> ReverseMap {
        let mut bagmap = HashMap::new();
        for bag in bags {
            for bagset in &bag.contents {
                let items = bagmap.entry(bagset.color.clone()).or_insert(Vec::new());
                items.push(bag.color.clone());
            }
        }
        ReverseMap(bagmap)
    }

    fn packable(&self, color: &str) -> HashSet<String> {
        let mut packable = HashSet::new();

        let mut found = HashSet::new();
        found.insert(color.to_string());

        while !found.is_empty() {
            let next_found: HashSet<String> = found.iter()
                .filter_map(|color| self.0.get(color))
                .flatten()
                .map(|color| color.clone())
                .collect();

            packable = next_found.union(&packable).cloned().collect();
            found = next_found;
        }

        packable
    }
}

fn main() {
    let path = env::args().nth(1).expect("no path to file");
    let bags = Bag::read(&path);

    let map = ReverseMap::from_bags(&bags);
    let packable = map.packable("shiny gold");

    println!("bags that can pack shiny gold: {:?}", packable);
    println!("number of packable bags: {}", packable.len());

    let shiny = bags.iter()
        .find(|b| b.color == "shiny gold")
        .expect("cannot find shiny gold");

    let unpacked = shiny.unpack(&bags);
    println!("bags inside shiny gold: {}", unpacked);
}
