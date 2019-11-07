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
}

/// A brainfuck program
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
            }
            true
        }
    }
}
