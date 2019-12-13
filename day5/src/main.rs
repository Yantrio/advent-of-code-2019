use computer::{Computer, IOMode};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").expect("failed to read input file");

    let mut c = Computer::from_string(&input[..], IOMode::Stdio);
    c.run();
}
