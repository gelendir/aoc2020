use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Instant};

#[derive(Debug, Clone)]
enum Instruction {
    NoOp(i16),
    Accumulate(i16),
    Jump(i16)
}

struct Interpreter {
    instructions: Vec<Instruction>,
    position: i16,
    accumulator: i16,
    visited: Vec<i16>
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
        let mut parts = text.split(" ");
        let op = parts.next().expect("op not found");
        let value = parts.next().expect("value not found");
        match op {
            "nop" => Instruction::NoOp(Self::parse_int(value)),
            "acc" => Instruction::Accumulate(Self::parse_int(value)),
            "jmp" => Instruction::Jump(Self::parse_int(value)),
            _ => panic!("unexpected instruction")
        }
    }
        
    fn parse_int(text: &str) -> i16 {
        let value = text[1..].parse().expect("cannot parse int value");
        if text.chars().nth(0) == Some('-') {
            value * -1
        } else {
            value
        }
    }

    fn invert(&self) -> Instruction {
        match self {
            Instruction::Jump(value) => Instruction::NoOp(*value),
            Instruction::NoOp(value) => Instruction::Jump(*value),
            _ => panic!("unexpected inversion")
        }
    }
}

impl Interpreter {

    fn new(instructions: Vec<Instruction>) -> Interpreter {
        Interpreter {
            instructions: instructions,
            position: 0,
            accumulator: 0,
            visited: Vec::new()
        }
    }

    fn execute(&mut self) -> i16 {
        while !self.can_continue() {
            self.execute_instruction()
        }
        self.accumulator
    }

    fn can_continue(&self) -> bool {
        self.is_terminated() || self.visited.contains(&self.position)
    }

    fn is_terminated(&self) -> bool {
        self.position as usize >= self.instructions.len()
    }

    fn execute_instruction(&mut self) {
        self.visited.push(self.position);

        match self.instructions[self.position as usize] {
            Instruction::NoOp(_) => self.position += 1,
            Instruction::Accumulate(value) => {
                self.accumulator += value;
                self.position += 1
            } 
            Instruction::Jump(value) => self.position += value
        }
    }
}

fn run_loop(path: &str) -> i16 {
    let instructions = Instruction::read(&path);
    let mut interpreter = Interpreter::new(instructions.clone());
    interpreter.execute()
}

fn run_terminate(path: &str) -> i16 {
    let instructions = Instruction::read(&path);
    instructions.iter()
        .enumerate()
        .filter(|(_, instruction)| match instruction {
            Instruction::NoOp(_) => true,
            Instruction::Jump(_) => true,
            _ => false
        })
        .filter_map(|(pos, instruction)| {
            let mut copy = instructions.clone();
            copy[pos] = instruction.invert();

            let mut interpreter = Interpreter::new(copy);
            let value = interpreter.execute();

            match interpreter.is_terminated() {
                true => Some(value),
                false => None
            }
        })
        .next()
        .expect("no interpretations terminated")
}

fn main() {
    let mode = env::args().nth(1).expect("no mode");
    let path = env::args().nth(2).expect("no path to file");

    match mode.as_str() {
        "loop" => println!("loop value {}", run_loop(&path)),
        "terminate" => println!("terminate value {}", run_terminate(&path)),
        "benchmark" => {
            let runs = 1000;
            let total: u128 = (0..runs)
                .map(|i| {
                    let start = Instant::now();
                    run_terminate(&path);
                    let end = Instant::now();

                    let nanos = end.duration_since(start).as_nanos();
                    println!("run {} nanos {}", i, nanos);
                    nanos
                })
                .sum();
            
            let average = total as f64 / runs as f64;
            println!("average execution time: {} ns", average);
        },
        _ => {}
    }
}
