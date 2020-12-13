use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;

#[derive(Debug, Clone, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied
}

#[derive(Debug)]
struct Map {
    seats: Vec<Vec<Seat>>,
    modified: bool
}

impl Seat {

    fn parse(symbol: char) -> Seat {
        match symbol {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!("unexpected seat symbol")
        }
    }

    fn to_char(&self) -> char {
        match self {
            Seat::Occupied => '#',
            Seat::Empty => 'L',
            Seat::Floor => '.'
        }
    }

    fn can_flip(&self, surrounding: &Vec<&Seat>) -> bool {
        match self {
            Seat::Empty => Self::flip_empty(surrounding),
            Seat::Occupied => Self::flip_occupied(surrounding),
            Seat::Floor => false
        }
    }

    fn flip_empty(surrounding: &Vec<&Seat>) -> bool {
        !surrounding.iter()
            .any(|s| **s == Seat::Occupied)
    }

    fn flip_occupied(surrounding: &Vec<&Seat>) -> bool {
        let occupied = surrounding.iter()
            .filter(|s| ***s == Seat::Occupied)
            .count();

        occupied >= 5
    }

    fn flip(&self) -> Seat {
        match self {
            Seat::Occupied => Seat::Empty,
            Seat::Empty => Seat::Occupied,
            Seat::Floor => Seat::Floor
        }
    }
}

impl Map {

    fn read(path: &str) -> Map {
        let file = File::open(path).expect("cannot open file");
        let buffer = BufReader::new(file);
        let seats = buffer.lines()
            .map(|result| result.expect("cannot read line"))
            .map(Self::parse_line)
            .collect();

        Map{
            modified: true,
            seats: seats
        }
    }

    fn parse_line(line: String) -> Vec<Seat> {
        line.chars().map(Seat::parse).collect()
    }

    fn rows(&self) -> usize {
        self.seats.len()
    }

    fn columns(&self) -> usize {
        self.seats[0].len()
    }

    fn get(&self, x: usize, y: usize) -> &Seat {
        &self.seats[y][x]
    }

    fn flip(&self) -> Map {
        let rows = self.rows();
        let columns = self.columns();
        let mut modified = false;

        let seats = (0..rows).map(|y| {
            (0..columns).map(|x| {

                let seat = self.get(x, y);
                let surrounding = self.nearest_seats(x, y);

                if seat.can_flip(&surrounding) {
                    modified = true;
                    seat.flip()
                } else {
                    seat.clone()
                }
            })
            .collect()
        })
        .collect();

        Map {
            seats: seats,
            modified: modified
        }
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<&Seat> {
        let rows = self.rows();
        let columns = self.columns();
        let mut adjacent = Vec::new();

        if x > 0 {
            adjacent.push(self.get(x - 1, y));
        }

        if x + 1 < columns {
            adjacent.push(self.get(x + 1, y));
        }

        if y > 0 {
            adjacent.push(self.get(x, y - 1));
        }

        if y + 1 < rows {
            adjacent.push(self.get(x, y + 1));
        }

        if x > 0 && y > 0 {
            adjacent.push(self.get(x - 1, y - 1));
        }

        if x + 1 < columns && y + 1 < rows {
            adjacent.push(self.get(x + 1, y + 1));
        }

        if x > 0 && y + 1 < rows {
            adjacent.push(self.get(x - 1, y + 1))
        }

        if x + 1 < columns && y > 0 {
            adjacent.push(self.get(x + 1, y - 1))
        }

        adjacent
    }

    fn nearest_seats(&self, x: usize, y: usize) -> Vec<&Seat> {
        let mut seats = Vec::new();

        if let Some(seat) = self.nearest_seat(x, y, 1, 0) {
            seats.push(seat);
        }

        if let Some(seat) = self.nearest_seat(x, y, -1, 0) {
            seats.push(seat);
        }

        if let Some(seat) = self.nearest_seat(x, y, 0, 1) {
            seats.push(seat);
        }

        if let Some(seat) = self.nearest_seat(x, y, 0, -1) {
            seats.push(seat);
        }

        if let Some(seat) = self.nearest_seat(x, y, 1, 1) {
            seats.push(seat);
        }

        if let Some(seat) = self.nearest_seat(x, y, -1, 1) {
            seats.push(seat);
        }

        if let Some(seat) = self.nearest_seat(x, y, 1, -1) {
            seats.push(seat);
        }

        if let Some(seat) = self.nearest_seat(x, y, -1, -1) {
            seats.push(seat);
        }

        seats
    }

    fn nearest_seat(&self, xstart: usize, ystart: usize, xdelta: i8, ydelta: i8) -> Option<&Seat> {
        let max_x = self.columns() as i8;
        let max_y = self.rows() as i8;

        let mut x = xstart as i8 + xdelta;
        let mut y = ystart as i8 + ydelta;

        while x >= 0 && x < max_x && y >= 0 && y < max_y {
            let seat = self.get(x as usize, y as usize);
            if *seat != Seat::Floor {
                return Some(seat)
            }
            x += xdelta;
            y += ydelta;
        }

        None
    }

    fn occupied(&self) -> usize {
        self.seats.iter()
            .map(|row| {
                row.iter()
                    .filter(|s| **s == Seat::Occupied)
                    .count()
            })
            .sum()
    }

    fn print(&self) {
        self.seats.iter().for_each(|row| {
            let text: String = row.iter()
                .map(|s| s.to_char())
                .collect();
            println!("{}", text);
        });
        println!("");
    }
}

fn main() {
    let path = env::args().nth(1).expect("no file path");
    let mut map = Map::read(&path);
    map.print();

    while map.modified {
        map = map.flip();
    }

    map.print();
    println!("{:?}", map.occupied());
}
