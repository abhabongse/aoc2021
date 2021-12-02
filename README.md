# Advent of Code 2021

Small code to solve problems at https://adventofcode.com/2021.  
Most of the code are written in Rust.

## How to run solutions

For example, to run the code for **Day 1: Sonar Sweep**:

```bash
$ cargo run --bin day01 inputs/day01.txt
```

You may also add `--release` flag for a more optimized binary:

```bash
$ cargo run --release --bin day01 inputs/day01.txt
```

## Approaches and considerations

1. My goal with Advent of Code 2021 is to dip my toe into Rust language and ecosystem, and to figure out how to transfer
   my code style into new language. Code performance and quality are important; submission speed is not.
2. I use AOC solution checker to validate the logic behind my initial solutions. However, the source files shown in this
   repo might be a version improved upon those solutions.
3. Unless the input data is simple, data definitions will be explicitly defined alongside
   with [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait.
4. Explicit type annotations are minimal. They are inferred and hinted inline by IDE most of the time, so I did not
   bother.
5. No mindless usage of [`Option::unwrap()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap)
   and [`Result::unwrap()`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap).
6. Rust external crates are fair game, unless the problem would have become too trivial.
