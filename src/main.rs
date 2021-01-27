use std::io;
use std::{env::args, fs::read_to_string};

use optimizing_bf::{interpreter::Interpreter, tokens::tokenize};

fn main() {
    let filename = args().nth(1).unwrap();
    let program_text = read_to_string(filename).unwrap();

    let program = tokenize(&program_text);
    let mut interpreter = Interpreter::new(program);

    let stdin = io::stdin();
    let stdout = io::stdout();

    interpreter.execute(stdin.lock(), stdout.lock());
}
