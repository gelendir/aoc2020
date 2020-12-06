use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const ROWS: (u32, u32) = (0, 127);
const COLUMNS: (u32, u32) = (0, 7);

#[derive(Debug)]
enum Split {
    Bottom,
    Top
}

#[derive(Debug)]
struct Seat {
    rows: Vec<Split>,
    columns: Vec<Split>
}

type Bounds = (u32, u32);

impl Split {

    fn reduce(&self, bounds: &Bounds) -> Bounds {
        let remaining = (bounds.1 - bounds.0) / 2;
        match self {
            Split::Bottom => (bounds.0, bounds.0 + remaining),
            Split::Top => (bounds.1 - remaining, bounds.1)
        }
    }
}

impl Seat {

    fn parse(text: String) -> Seat {
        let rows = text.chars()
            .filter_map(|c| match c {
                'F' => Some(Split::Bottom),
                'B' => Some(Split::Top),
                _ => None
            })
            .collect();

        let columns = text.chars()
            .filter_map(|c| match c {
                'L' => Some(Split::Bottom),
                'R' => Some(Split::Top),
                _ => None
            })
            .collect();

        Seat {
            rows: rows,
            columns: columns
        }
    }

    fn seat_id(&self) -> u32 {
        let bounds = self.calculate();
        bounds.0 * 8 + bounds.1
    }

    fn calculate(&self) -> Bounds {
        let row = Self::reduce(&self.rows, ROWS);
        let column = Self::reduce(&self.columns, COLUMNS);
        (row, column)
    }

    fn reduce(splits: &Vec<Split>, value: Bounds) -> u32 {
        let mut value = value.clone();
        for split in splits {
            value = split.reduce(&value);
        }

        match splits.last() {
            Some(Split::Bottom) => value.0,
            Some(Split::Top) => value.1,
            _ => panic!("no splits in list")
        }
    }

}

fn main() {
    let path = env::args().nth(1).expect("no path to input file");
    let file = File::open(path).expect("cannot open file");
    let buffer = BufReader::new(file);

    let mut seat_ids: Vec<u32> = buffer.lines()
        .map(|result| result.expect("cannot read line"))
        .map(Seat::parse)
        .map(|s| s.seat_id())
        .collect();

    seat_ids.sort();
    
    println!("highest seat id {}", seat_ids.last().unwrap());

    for (i, seat_id) in seat_ids[1..].iter().enumerate() {
        let previous = seat_ids[i];
        if previous + 1 != *seat_id {
            println!("vacant seat id {}", seat_id - 1);
        }
    }
}
