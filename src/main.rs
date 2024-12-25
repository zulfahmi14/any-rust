use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let input = String::from("Hello fellow Rustaceans!");
    let width = input.chars().count();
    let writer = BufWriter::new(stdout.lock());

    say(&input, width, writer).unwrap();
}
