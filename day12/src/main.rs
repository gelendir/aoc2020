use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Unit = i32;

enum Instruction {
    North(Unit),
    East(Unit),
    South(Unit),
    West(Unit),
    Left(Unit),
    Right(Unit),
    Forward(Unit)
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

struct Coordinate {
    x: Unit,
    y: Unit
}

struct Ship {
    direction: Direction,
    position: Coordinate,
    waypoint: Coordinate
}

impl Instruction {

    fn read(path: &str) -> Vec<Instruction> {
        let file = File::open(path).expect("cannot open file");
        let buffer = BufReader::new(file);
        buffer.lines()
            .map(|result| result.expect("cannot read line"))
            .map(Self::parse)
            .collect()
    }

    fn parse(text: String) -> Instruction {
        let symbol = text.chars()
            .nth(0)
            .expect("no instruction symbol");

        let value: Unit = text[1..]
            .parse()
            .expect("cannot parse instruction value");

        match symbol {
            'N' => Instruction::North(value),
            'E' => Instruction::East(value),
            'S' => Instruction::South(value),
            'W' => Instruction::West(value),
            'L' => Instruction::Left(value),
            'R' => Instruction::Right(value),
            'F' => Instruction::Forward(value),
            _ => panic!("unexpected instruction")
        }
    }

}

impl Direction {

    fn turn_left(&mut self, degrees: &Unit) {
        let mut degrees = *degrees;
        while degrees > 0 {
            *self = match self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North
            };
            degrees -= 90;
        }
    }

    fn turn_right(&mut self, degrees: &Unit) {
        let mut degrees = *degrees;
        while degrees > 0 {
            *self = match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            };
            degrees -= 90;
        }
    }

}

impl Coordinate {

    fn new(x: Unit, y: Unit) -> Coordinate {
        Coordinate {
            x: x,
            y: y
        }
    }

    fn advance(&mut self, direction: &Direction, value: &Unit) {
        match direction {
            Direction::North => self.y += value,
            Direction::South => self.y -= value,
            Direction::East => self.x += value,
            Direction::West => self.x -= value
        }
    }

    fn translate(&mut self, other: &Coordinate, factor: Unit) {
        self.x += other.x * factor;
        self.y += other.y * factor;
    }

    fn rotate_right(&mut self, mut degrees: Unit) {
        while degrees > 0 {
            let y = self.y;
            self.y = -self.x;
            self.x = y;
            degrees -= 90;
        }
    }

    fn rotate_left(&mut self, mut degrees: Unit) {
        while degrees > 0 {
            let x = self.x;
            self.x = -self.y;
            self.y = x;
            degrees -= 90;
        }
    }

    fn manhattan_distance(&self) -> Unit {
        self.x.abs() + self.y.abs()
    }

}

impl Ship {

    fn new() -> Ship {
        Ship {
            direction: Direction::East,
            position: Coordinate::new(0, 0),
            waypoint: Coordinate::new(10, 1)
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(value) => self.position.advance(&Direction::North, value),
            Instruction::South(value) => self.position.advance(&Direction::South, value),
            Instruction::East(value) => self.position.advance(&Direction::East, value),
            Instruction::West(value) => self.position.advance(&Direction::West, value),
            Instruction::Left(value) => self.direction.turn_left(value),
            Instruction::Right(value) => self.direction.turn_right(value),
            Instruction::Forward(value) => self.position.advance(&self.direction, value)
        }
    }

    fn apply_waypoint(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(value) => self.waypoint.advance(&Direction::North, value),
            Instruction::South(value) => self.waypoint.advance(&Direction::South, value),
            Instruction::East(value) => self.waypoint.advance(&Direction::East, value),
            Instruction::West(value) => self.waypoint.advance(&Direction::West, value),
            Instruction::Left(value) => self.waypoint.rotate_left(*value),
            Instruction::Right(value) => self.waypoint.rotate_right(*value),
            Instruction::Forward(value) => self.position.translate(&self.waypoint, *value)
        }

    }

    fn manhattan_distance(&self) -> Unit {
        self.position.manhattan_distance()
    }
}

fn main() {
    let mode = env::args().nth(1).expect("no mode");
    let path = env::args().nth(2).expect("no file path");

    let instructions = Instruction::read(&path);
    let mut ship = Ship::new();

    if mode == "part1" {
        instructions.iter()
            .for_each(|instruction| ship.apply(instruction));
    } else {
        instructions.iter()
        .for_each(|instruction| ship.apply_waypoint(instruction));
    }

    println!("{}", ship.manhattan_distance())
}
