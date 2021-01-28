use std::convert::TryInto;
use std::io;
use std::{env::args, fs::read_to_string};

use optimizing_bf::{interpreter::Machine, optimize, syntax::BFSyntax, tokens::tokenize};

fn main() {
    let filename = args().nth(1).unwrap();
    let program_text = read_to_string(filename).unwrap();

    let program = tokenize(&program_text);
    let program_syntax: BFSyntax = optimize::fold_adjacent_constants(program.try_into().unwrap());

    let mut machine = Machine::new();
    machine.execute(
        dbg!(program_syntax),
        &mut io::stdin().lock(),
        &mut io::stdout().lock(),
    )
}
