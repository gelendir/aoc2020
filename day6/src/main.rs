use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

type Person = HashSet<char>;

#[derive(Debug)]
struct Group(Vec<Person>);

fn parse_person(text: &str) -> Person {
    text.chars().collect()
}

impl Group {

    fn new() -> Group {
        Group(Vec::new())
    }

    fn add(&mut self, person: Person) {
        self.0.push(person);
    }

    fn read(path: &str) -> Vec<Group> {
        let file = File::open(path).expect("cannot open file");
        let buffer = BufReader::new(file);

        let mut groups = vec![Group::new()];

        buffer.lines()
            .map(|result| result.expect("cannot read line"))
            .for_each(|line| match line.as_str() {
                "" => groups.push(Group::new()),
                text => {
                    let last = groups.last_mut().expect("no groups to add questions");
                    last.add(parse_person(text))
                }
            });

        groups
    }

    fn total_questions(&self) -> usize {
        self.0.iter()
            .fold(HashSet::new(), |total, person| {
                total.union(&person).cloned().collect()
            })
            .len()
    }

    fn common_questions(&self) -> usize {
        let base = self.0[0].clone();
        self.0[1..].iter()
            .fold(base, |total, person| {
                total.intersection(&person).cloned().collect()
            })
            .len()
    }
}

fn main() {
    let path = env::args().nth(1).expect("no path to file");
    let groups = Group::read(&path);

    let total: usize = groups.iter()
        .map(|g| g.total_questions())
        .sum();

    let common: usize = groups.iter()
        .map(|g| g.common_questions())
        .sum();

    println!("total {}", total);
    println!("common {}", common);
}