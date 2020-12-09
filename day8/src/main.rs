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

struct Branch {
    position: usize,
    instruction: Instruction
}

struct Interpreter<'a> {
    instructions: &'a Vec<Instruction>,
    position: usize,
    accumulator: i16,
    branch: Option<Branch>,
    visited: Vec<bool>
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

impl<'a> Interpreter<'a> {

    fn new(instructions: &Vec<Instruction>) -> Interpreter {
        let capacity = instructions.len();
        Interpreter {
            instructions: instructions,
            position: 0,
            accumulator: 0,
            branch: None,
            visited: vec![false; capacity]
        }
    }

    fn branch(&self) -> (i16, bool) {
        let instruction = self.instructions[self.position].invert();

        let mut interpreter = Interpreter {
            instructions: &self.instructions,
            position: self.position,
            accumulator: self.accumulator,
            branch: Some(Branch{
                position: self.position,
                instruction: instruction
            }),
            visited: self.visited.clone()
        };

        let result = interpreter.execute();
        (result, interpreter.is_terminated())
    }

    fn execute(&mut self) -> i16 {
        while self.can_continue() {
            if self.branch.is_none() {
                match self.instructions[self.position] {
                    Instruction::Jump(_) | Instruction::NoOp(_) => {
                        let (result, terminated) = self.branch();
                        if terminated {
                            return result
                        }
                    },
                    _ => {}
                }
            }
            self.execute_instruction();
        }

        return self.accumulator
    }

    fn can_continue(&self) -> bool {
        ! (self.is_terminated() || self.visited[self.position])
    }

    fn is_terminated(&self) -> bool {
        self.position as usize >= self.instructions.len()
    }

    fn execute_instruction(&mut self) {
        self.visited[self.position] = true;

        let instruction = match &self.branch {
            Some(b) if b.position == self.position => &b.instruction,
            _ => &self.instructions[self.position]
        }; 

        match instruction {
            Instruction::NoOp(_) => self.position += 1,
            Instruction::Accumulate(value) => {
                self.accumulator += value;
                self.position += 1
            } 
            Instruction::Jump(value) => match value.is_positive() {
                true => self.position += *value as usize,
                false => self.position -= value.abs() as usize
            }
        }
    }
}

fn run(path: &str) -> i16 {
    let instructions = Instruction::read(&path);
    let mut interpreter = Interpreter::new(&instructions);
    interpreter.execute()
}

fn main() {
    let mode = env::args().nth(1).expect("no mode");
    let path = env::args().nth(2).expect("no path to file");

    match mode.as_str() {
        "run" => println!("terminate value {}", run(&path)),
        "benchmark" => {
            let runs = 1000;
            let total: u128 = (0..runs)
                .map(|i| {
                    let start = Instant::now();
                    run(&path);
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
