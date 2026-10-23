# optimizing-bf

An optimizing interpreter for [Brainfuck](https://esolangs.org/wiki/Brainfuck),
written in Rust.

Instead of executing the program text directly, it parses the program into a
syntax tree, runs optimization passes over the tree, and then executes the
optimized tree.

## Optimizations

- **Run folding.** Runs of `+`, `-`, `<` and `>` collapse into single operations
  with a count, such as `+++++` becoming `Inc(5)`.
- **Constant coalescing.** Adjacent increments, decrements and sets on a cell are
  merged. A cell that is overwritten by a read is not touched first.
- **Zeroing loops.** Loops like `[-]` that just zero the current cell become a
  single `Set(0)`.
- **Dead loop removal.** A loop at the very start of the program, or directly after
  another loop, can never run, because the current cell is zero when it is
  reached. Those loops are removed.

## Running

```sh
cargo run --release -- helloworld.bf
```

The repo includes `helloworld.bf`. Programs read from standard input and write to
standard output. The binary also prints the optimized syntax tree with `dbg!` before
running the program.

## Layout

| File | Contents |
| --- | --- |
| `src/tokens.rs` | Tokenizer |
| `src/syntax.rs` | Syntax tree, built from tokens |
| `src/optimize.rs` | Optimization passes |
| `src/interpreter.rs` | The machine that runs the tree |

## License

Mozilla Public License 2.0. See [LICENSE](LICENSE).
