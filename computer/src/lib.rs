#[macro_use]
extern crate lazy_static;

pub use self::operation::Mode;
pub use self::operation::OpCode;
pub use self::operation::Operation;
mod operation;

use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};
use std::{thread, time};

const TIMEOUT: u64 = 1000;

#[derive(Debug)]
pub enum IOMode {
    Buffer,
    Stdio,
}

#[derive(Debug)]
pub struct Computer {
    pub mem: Vec<i64>,
    pub ip: usize,
    pub iomode: IOMode,
    pub input_buf: VecDeque<i64>,
    pub output_buf: VecDeque<i64>,
    pub break_on_output: bool,
    pub log_prefix: String,
    pub enable_logger: bool,
    pub relative_base: i64,
}

impl Computer {
    pub fn from_string(input: &str, iomode: IOMode) -> Computer {
        let mut c = Computer {
            mem: input.split(",").map(|s| s.parse().unwrap()).collect(),
            ip: 0,
            input_buf: VecDeque::new(),
            output_buf: VecDeque::new(),
            iomode: iomode,
            break_on_output: false,
            log_prefix: "".to_string(),
            enable_logger: false,
            relative_base: 0,
        };
        let mut extramem = (0..65536).map(|_| 0).collect::<Vec<i64>>();
        c.mem.append(&mut extramem);
        c
    }

    pub fn input_to_buffer(&mut self, inp: i64) {
        self.input_buf.push_back(inp);
    }

    fn read_from_input_buffer(&mut self) -> i64 {
        let timeout = time::Instant::now() + time::Duration::from_millis(TIMEOUT);
        loop {
            if !self.input_buf.is_empty() {
                return self
                    .input_buf
                    .pop_front()
                    .expect("Error reading value from input buffer");
            }
            match time::Instant::now() < timeout {
                true => thread::sleep(time::Duration::from_millis(10)),
                false => panic!("Timed out waiting for input buffer"),
            }
        }
    }

    pub fn output_from_buffer(&mut self) -> i64 {
        let timeout = time::Instant::now() + time::Duration::from_millis(TIMEOUT);
        loop {
            if !self.output_buf.is_empty() {
                return self
                    .output_buf
                    .pop_front()
                    .expect("Error reading value from output buffer");
            }
            match time::Instant::now() < timeout {
                true => thread::sleep(time::Duration::from_millis(10)),
                false => panic!("Timed out waiting for output buffer"),
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let op = Operation::from_computer(self);
            let length = Operation::get_length(&op.op_code);
            let orig_ip = self.ip;

            if self.enable_logger {
                //todo: log nicely
                self.log(format!("{:?}", op));
            }

            match op.op_code {
                OpCode::Add => self.add(op),
                OpCode::Mul => self.mul(op),
                OpCode::Input => self.input(op),
                OpCode::Output => {
                    self.output(op);
                    if self.break_on_output {
                        self.ip = self.ip + length;
                        break;
                    }
                }
                OpCode::JumpIfTrue => self.jump_if_true(op),
                OpCode::JumpIfFalse => self.jump_if_false(op),
                OpCode::Lessthan => self.less_than(op),
                OpCode::Equals => self.equals(op),
                OpCode::OffsetBase => self.offset_base(op),
                OpCode::End => {
                    break;
                }
            };
            if self.ip == orig_ip {
                self.increment_ip(length);
            }
        }
    }

    fn log(&mut self, str: String) {
        println!("{} -> {}", self.log_prefix, str);
    }

    fn increment_ip(&mut self, size: usize) {
        self.ip = self.ip + size;
    }

    pub fn is_running(&mut self) -> bool {
        self.mem[self.ip] != 99
    }

    fn read_stdin() -> i64 {
        let mut s = String::new();
        print!("Please enter a number: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("incorrect number");

        let trimmed = s.trim();
        match trimmed.parse::<i64>() {
            Ok(i) => return i,
            Err(..) => panic!("Not an integer: {}", trimmed),
        }
    }

    fn offset_base(&mut self, op: Operation) {
        self.relative_base = self.relative_base + self.get(&op, 1);
    }

    fn input<'a>(&mut self, op: Operation) {
        let val = match self.iomode {
            IOMode::Stdio => Computer::read_stdin(),
            IOMode::Buffer => self.read_from_input_buffer(),
        };
        self.set(&op, 1, val);
    }

    fn output<'a>(&mut self, op: Operation) {
        let val = self.get(&op, 1);
        match self.iomode {
            IOMode::Stdio => println!("Output: {}", val),
            IOMode::Buffer => self.output_buf.push_back(val),
        };
    }

    fn add(&mut self, op: Operation) {
        let res = self.get(&op, 1) + self.get(&op, 2);
        self.set(&op, 3, res);
    }

    fn mul(&mut self, op: Operation) {
        let res = self.get(&op, 1) * self.get(&op, 2);
        self.set(&op, 3, res);
    }

    fn jump_if_true(&mut self, op: Operation) {
        match self.get(&op, 1) != 0 {
            true => self.ip = self.get(&op, 2) as usize,
            false => self.increment_ip(Operation::get_length(&op.op_code)),
        }
    }

    fn jump_if_false(&mut self, op: Operation) {
        match self.get(&op, 1) == 0 {
            true => self.ip = self.get(&op, 2) as usize,
            false => self.increment_ip(Operation::get_length(&op.op_code)),
        }
    }

    fn less_than(&mut self, op: Operation) {
        let res = self.get(&op, 1) < self.get(&op, 2);
        self.set(&op, 3, if res == true { 1 } else { 0 });
    }

    fn equals(&mut self, op: Operation) {
        let res = self.get(&op, 1) == self.get(&op, 2);
        self.set(&op, 3, if res == true { 1 } else { 0 });
    }

    fn get_mode<'a>(&mut self, op: &'a Operation, parameter: i64) -> &'a Mode {
        match parameter {
            1 => &op.modes.0,
            2 => &op.modes.1,
            3 => &op.modes.2,
            _ => panic!("unknown paramater number, must be 1 2 or 3"),
        }
    }

    fn set(&mut self, op: &Operation, parameter: i64, value: i64) {
        let addr = op.data[parameter as usize];
        match self.get_mode(op, parameter) {
            Mode::Immediate => panic!("Cannot set immediately!"),
            Mode::Position => self.mem[addr as usize] = value,
            Mode::Relative => self.mem[(addr + self.relative_base) as usize] = value,
        }
    }

    fn get(&mut self, op: &Operation, parameter: i64) -> i64 {
        let v = op.data[parameter as usize];

        match self.get_mode(op, parameter) {
            Mode::Immediate => v,
            Mode::Position => self.mem[v as usize],
            Mode::Relative => self.mem[(v + self.relative_base) as usize],
        }
    }
}
