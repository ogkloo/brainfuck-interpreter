use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

/// List of brainfuck instructions
/// As detailed in https://esolangs.org/wiki/Brainfuck
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add,
    Sub,
    Right,
    Left,
    Branch,
    End,
    Print,
    Input,
    Noop,
}

#[derive(Debug, PartialEq)]
pub struct Tape {
    storage: Vec<(i8, usize)>,
    position: usize,
    branches: Vec<usize>,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
}

impl Tape {
    pub fn run(&mut self) {
        while self.handle_instruction() {
            println!("{:?}", self);
        }
    }

    pub fn handle_instruction(&mut self) -> bool {
        if self.instruction_pointer >= self.instructions.len() {
            false
        } else {
            match self.instructions[self.instruction_pointer] {
                Instruction::Add => {
                    match self
                        .storage
                        .iter()
                        .position(|&elem| elem.1 == self.position)
                    {
                        Some(index) => self.storage[index].0 += 1,
                        None => self.storage.push((1, self.position)),
                    }
                    self.instruction_pointer += 1;
                }
                Instruction::Sub => {
                    match self
                        .storage
                        .iter()
                        .position(|&elem| elem.1 == self.position)
                    {
                        Some(index) => self.storage[index].0 -= 1,
                        None => self.storage.push((-1, self.position)),
                    }
                    self.instruction_pointer += 1;
                }
                Instruction::Right => {
                    self.position += 1;
                    self.instruction_pointer += 1;
                }
                Instruction::Left => {
                    if self.position >= 1 {
                        self.position -= 1;
                    }
                    self.instruction_pointer += 1;
                }
                Instruction::Print => {
                    match self
                        .storage
                        .iter()
                        .position(|&elem| elem.1 == self.position)
                    {
                        Some(index) => println!("{}", self.storage[index].0),
                        None => println!("0"),
                    }
                    self.instruction_pointer += 1;
                }
                Instruction::Input => {
                    let mut input = String::new();
                    match self
                        .storage
                        .iter()
                        .position(|&elem| elem.1 == self.position)
                    {
                        Some(index) => match io::stdin().read_line(&mut input) {
                            Ok(_) => {
                                self.storage[index].0 = match input.trim().parse::<i8>() {
                                    Ok(r) => r,
                                    Err(_) => panic!("Input error (step 1)"),
                                }
                            }
                            Err(_) => panic!("Input error (read line)"),
                        },
                        None => match io::stdin().read_line(&mut input) {
                            Ok(_) => self.storage.push((
                                match input.trim().parse::<i8>() {
                                    Ok(r) => r,
                                    Err(_) => panic!("Input error"),
                                },
                                self.position,
                            )),
                            Err(_) => panic!("Error parsing input"),
                        },
                    }
                    self.instruction_pointer += 1;
                }
                Instruction::Branch => {
                    self.branches.push(self.instruction_pointer);
                    self.instruction_pointer += 1;
                }
                Instruction::End => {
                    match self
                        .storage
                        .iter()
                        .position(|&elem| elem.1 == self.position)
                    {
                        Some(index) => {
                            if self.storage[index].0 == 0 {
                                match self.branches.pop() {
                                    Some(branch) => self.instruction_pointer = branch,
                                    None => panic!("Jump to non-existent branch statement."),
                                }
                            } else {
                                self.instruction_pointer += 1;
                            }
                        }
                        None => match self.branches.pop() {
                            Some(branch) => self.instruction_pointer = branch,
                            None => panic!("Jump to non-existent branch statement."),
                        },
                    }
                }
                Instruction::Noop => {
                    self.instruction_pointer += 1;
                }
            }
            true
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() == 2 {
        args[1].clone()
    } else {
        "test.b".to_string()
    };
    let mut input_file = File::open(&filename)?;
    let mut program = String::new();
    input_file.read_to_string(&mut program)?;
    let mut tape = Tape {
        storage: Vec::new(),
        position: 0,
        branches: Vec::new(),
        instructions: vec![],
        instruction_pointer: 0,
    };
    for command in program.chars() {
        tape.instructions.push(match command {
            '+' => Instruction::Add,
            '-' => Instruction::Sub,
            '>' => Instruction::Right,
            '<' => Instruction::Left,
            '.' => Instruction::Print,
            ',' => Instruction::Input,
            '[' => Instruction::Branch,
            ']' => Instruction::End,
            _ => Instruction::Noop,
        })
    }
    tape.run();
    Ok(())
}
