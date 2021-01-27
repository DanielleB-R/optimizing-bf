use std::io;

use crate::tokens::BFToken;

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

    fn interpret(&mut self, mut input: impl io::Read, mut output: impl io::Write) {
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
                let mut buf: [u8; 1] = [0];
                let amount = input.read(&mut buf).unwrap();
                if amount == 0 {
                    panic!("input is over");
                }
                self.cells[self.position] = buf[0];
            }
            BFToken::Write => {
                output.write(&[self.cells[self.position]]).unwrap();
                output.flush().unwrap();
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
