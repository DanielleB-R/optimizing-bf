use std::io;

use crate::{syntax::BFSyntax, tokens::BFToken};

#[derive(Debug, Clone)]
pub struct Interpreter {
    program: Vec<BFToken>,
    ip: usize,
    cells: Vec<u8>,
    position: usize,
}

impl Interpreter {
    pub fn new(program: Vec<BFToken>) -> Self {
        Self {
            program,
            ip: 0,
            cells: vec![0; 30_000],
            position: 0,
        }
    }

    pub fn execute(&mut self, mut input: impl io::Read, mut output: impl io::Write) {
        while self.ip < self.program.len() {
            self.interpret(&mut input, &mut output);
            self.ip += 1;
        }
    }

    fn interpret(&mut self, input: impl io::Read, output: impl io::Write) {
        match self.program[self.ip] {
            BFToken::Inc => {
                self.cells[self.position] += 1;
            }
            BFToken::Dec => {
                self.cells[self.position] -= 1;
            }
            BFToken::Left => {
                self.position -= 1;
            }
            BFToken::Right => {
                self.position += 1;
            }
            BFToken::BeginLoop => {
                if self.cells[self.position] == 0 {
                    self.advance_to_end();
                }
            }
            BFToken::EndLoop => {
                if self.cells[self.position] != 0 {
                    self.retreat_to_start();
                }
            }
            BFToken::Read => {
                self.cells[self.position] = read_byte(input);
            }
            BFToken::Write => {
                write_byte(output, self.cells[self.position]);
            }
        }
    }

    fn advance_to_end(&mut self) {
        let mut depth = 0;
        self.ip += 1;
        while depth != 0 || self.program[self.ip] != BFToken::EndLoop {
            match self.program[self.ip] {
                BFToken::BeginLoop => depth += 1,
                BFToken::EndLoop => depth -= 1,
                _ => {}
            }
            self.ip += 1
        }
    }

    fn retreat_to_start(&mut self) {
        let mut depth = 0;
        self.ip -= 1;
        while depth != 0 || self.program[self.ip] != BFToken::BeginLoop {
            match self.program[self.ip] {
                BFToken::BeginLoop => depth -= 1,
                BFToken::EndLoop => depth += 1,
                _ => {}
            }
            self.ip -= 1
        }
    }
}

fn read_byte(mut input: impl io::Read) -> u8 {
    let mut buf: [u8; 1] = [0];
    let amount = input.read(&mut buf).unwrap();
    if amount == 0 {
        panic!("input is finished");
    }
    buf[0]
}

fn write_byte(mut output: impl io::Write, value: u8) {
    output.write(&[value]).unwrap();
    output.flush().unwrap();
}

#[derive(Debug, Clone)]
pub struct Machine {
    cells: Vec<u8>,
    position: usize,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    pub fn new() -> Self {
        Self {
            cells: vec![0; 30_000],
            position: 0,
        }
    }

    pub fn execute(
        &mut self,
        program: BFSyntax,
        input: &mut impl io::Read,
        output: &mut impl io::Write,
    ) {
        match program {
            BFSyntax::Root(instructions) => {
                for instruction in instructions {
                    self.execute(instruction, input, output);
                }
            }
            BFSyntax::Right(n) => {
                self.position += n;
            }
            BFSyntax::Left(n) => {
                self.position -= n;
            }
            BFSyntax::Inc(n) => {
                self.cells[self.position] = self.cells[self.position].wrapping_add(n);
            }
            BFSyntax::Dec(n) => {
                self.cells[self.position] = self.cells[self.position].wrapping_sub(n);
            }
            BFSyntax::Set(n) => {
                self.cells[self.position] = n;
            }
            BFSyntax::Read => {
                self.cells[self.position] = read_byte(input);
            }
            BFSyntax::Write => write_byte(output, self.cells[self.position]),
            BFSyntax::Loop(instructions) => {
                while self.cells[self.position] != 0 {
                    for instruction in instructions.clone() {
                        self.execute(instruction, input, output);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_instructions() {
        let mut machine = Machine::new();

        let mut input = io::empty();
        let mut output = io::sink();
        machine.execute(BFSyntax::Inc(1), &mut input, &mut output);
        assert_eq!(machine.cells[0], 1);

        machine.execute(BFSyntax::Right(1), &mut input, &mut output);
        assert_eq!(machine.position, 1);

        machine.execute(BFSyntax::Dec(1), &mut input, &mut output);
        assert_eq!(machine.cells[1], 255);
    }

    #[test]
    fn test_loop() {
        let mut machine = Machine::new();
        let mut input = io::empty();
        let mut output = io::sink();
        machine.cells[0] = 5;

        machine.execute(
            BFSyntax::Loop(vec![
                BFSyntax::Dec(1),
                BFSyntax::Right(1),
                BFSyntax::Left(1),
            ]),
            &mut input,
            &mut output,
        );
        assert_eq!(machine.cells[0], 0);
    }
}
