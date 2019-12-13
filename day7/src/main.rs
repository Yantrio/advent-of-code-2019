#[macro_use]
extern crate lazy_static;

use computer::{Computer, IOMode};
use permutohedron::Heap;
use std::fs::read_to_string;

fn main() {
    let max = Heap::new(&mut vec![0, 1, 2, 3, 4])
        .map(run)
        .max()
        .expect("something is broken, enjoy debugging");

    println!("Solution Part 1 : {:?}", max);

    let max = Heap::new(&mut vec![5, 6, 7, 8, 9])
        .map(run)
        .max()
        .expect("something is broken, enjoy debugging");

    println!("Solution Part 2: {:?}", max);
}

fn run(inputs: Vec<i32>) -> i32 {
    let mut amps: Vec<Computer> = (0..5).map(|i| create_computer(i.to_string())).collect();
    for i in 0..5 {
        amps[i].input_to_buffer(inputs[i]);
    }

    amps[0].input_to_buffer(0);

    loop {
        amps[0].run();
        for i in 1..5 {
            let res = amps[i - 1].output_from_buffer();
            amps[i].input_to_buffer(res);
            amps[i].run();
        }

        let output = amps[4].output_from_buffer();

        match amps[0].is_running() {
            true => amps[0].input_to_buffer(output),
            false => return output,
        }
    }
}

fn create_computer(name: String) -> computer::Computer {
    lazy_static! {
        static ref INPUT: String = read_to_string("input").expect("failed to read input file");
    }
    let mut c = Computer::from_string(&INPUT[..], IOMode::Buffer);
    c.log_prefix = format!("Amp {}", name);
    c.enable_logger = false;
    c.break_on_output = true;
    c
}
