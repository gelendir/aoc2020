use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Bus {
    Id(i64),
    None
}

enum Mode {
    Earliest,
    Offset
}

impl Mode {

    fn execute(&self, departure: i64, ids: &Vec<Bus>) -> i64 {
        match self {
            Mode::Earliest => Self::execute_earliest(departure, ids),
            Mode::Offset => Self::execute_offset(ids)
        }
    }

    fn execute_earliest(departure: i64, ids: &Vec<Bus>) -> i64 {
        let mut schedules: Vec<(i64, i64)> = ids.iter()
            .filter_map(|bus| match bus {
                Bus::Id(id) => {
                    let next_departure = (departure / id + 1) * id;
                    Some((*id, next_departure))
                },
                Bus::None => None
            })
            .collect();

        schedules.sort_by_key(|(_, next)| *next);

        let (id, next_departure) = schedules[0];
        (next_departure - departure) * id
    }

    fn execute_offset(ids: &Vec<Bus>) -> i64 {
        let (pos, max) = ids.iter()
            .enumerate()
            .filter_map(|(delta, bus)| match bus {
                Bus::Id(id) => Some((delta as i64, *id)),
                Bus::None => None
            })
            .max_by_key(|(_, id)| *id)
            .expect("max increment not found");

        let ids: Vec<(i64, i64)> = ids.iter()
            .enumerate()
            .filter_map(|(delta, bus)| match bus {
                Bus::Id(id) if *id != max => Some((delta as i64 - pos, *id)),
                _ => None
            })
            .collect();

        println!("ids {:?}", ids);

        let increment = max;
        println!("increment {}", increment);

        let mut departure = max;
        while !Self::offsets_ok(departure, &ids) {
            departure += increment;
        }

        departure - pos
    }

    fn offsets_ok(departure: i64, ids: &Vec<(i64, i64)>) -> bool {
        ids.iter()
            .all(|(delta, id)| {
                (departure + delta) % id == 0
            })
    }

}

fn read(path: &str) -> (i64, Vec<Bus>) {
    let file = File::open(path).expect("cannot open file");
    let buffer = BufReader::new(file);
    let mut lines = buffer.lines();

    let departure = lines.next()
        .expect("no departure line")
        .expect("cannot read departure line")
        .parse()
        .expect("cannot parse departure");

    let ids = lines.next()
        .expect("no bus line")
        .expect("cannot read bus line")
        .split(",")
        .map(|id| {
            match id {
                "x" => Bus::None,
                number => Bus::Id(number.parse().expect("cannot parse bus ID"))
            }
        })
        .collect();

    (departure, ids)
}

fn main() {
    let mode = env::args().nth(1).expect("no mode");
    let path = env::args().nth(2).expect("no file path");
    let (departure, ids) = read(&path);

    let mode = match mode.as_str() {
        "earliest" => Mode::Earliest,
        "offset" => Mode::Offset,
        _ => panic!("unexpected mode")
    };

    let result = mode.execute(departure, &ids);
    println!("{}", result)
}
