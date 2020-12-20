use std::env;
use std::fs;

#[derive(Debug)]
struct Trail {
    spoken: [usize; 2]
}

impl Trail {

    fn new() -> Trail {
        Trail {
            spoken: [0, 0]
        }
    }

    fn from_turn(turn: usize) -> Trail {
        Trail {
            spoken: [0, turn+1]
        }
    }

    fn add_turn(&mut self, turn: usize) {
        self.spoken[0] = self.spoken[1];
        self.spoken[1] = turn+1;
    }

    fn age(&self) -> usize {
        if self.spoken[0] == 0 {
            0
        } else {
            self.spoken[1] - self.spoken[0]
        }
    }

}

fn main() {
    let maxturn: usize = env::args().nth(1).expect("maxturn missing")
        .parse()
        .expect("cannot parse maxturn");

    let path = env::args().nth(2).expect("file path missing");
    let text = fs::read_to_string(&path).expect("cannot read file");

    let numbers: Vec<usize> = text.split(",")
        .map(|number| number.parse().expect("cannot parse number"))
        .collect();

    let mut spoken: usize = *numbers.last().expect("no initial numbers");
    let capacity = numbers
        .iter()
        .max()
        .expect("no initial numbers")
        + 1;

    let mut trail = Vec::new();
    trail.resize_with(capacity, || Trail::new());

    numbers.iter()
        .enumerate()
        .for_each(|(i, n)| {
            trail[*n] = Trail::from_turn(i)
        });

    let start = numbers.len();
    (start..maxturn).for_each(|turn| {
        spoken = trail[spoken].age();

        if trail.len() <= spoken {
            trail.resize_with(spoken+1, || Trail::new());
        }
        trail[spoken].add_turn(turn);
    });

    println!("{}", spoken);
}
