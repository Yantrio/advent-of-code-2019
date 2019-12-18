#[macro_use]
extern crate lazy_static;

use crossbeam_channel::bounded;
use crossbeam_channel::{unbounded, Receiver, Sender};

pub use self::operation::Mode;
pub use self::operation::OpCode;
pub use self::operation::Operation;
mod operation;

use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone)]
pub enum IOMode {
    Buffer,
    Stdio,
    Channel,
}

type Buffer = VecDeque<i64>;

#[derive(Debug, Clone)]
pub struct Memory {
    pub input_buffer: Buffer,
    pub output_buffer: Buffer,
    pub memory: Vec<i64>,
    pub relative_base: i64,
    pub instruction_pointer: usize,
}

impl Memory {
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
            Mode::Position => self.memory[addr as usize] = value,
            Mode::Relative => self.memory[(addr + self.relative_base) as usize] = value,
        }
    }

    fn get(&mut self, op: &Operation, parameter: i64) -> i64 {
        let v = op.data[parameter as usize];

        match self.get_mode(op, parameter) {
            Mode::Immediate => v,
            Mode::Position => self.memory[v as usize],
            Mode::Relative => self.memory[(v + self.relative_base) as usize],
        }
    }

    fn from_string(input: &str) -> Memory {
        let mut extramem = (0..65536).map(|_| 0).collect::<Vec<i64>>();

        let mut m = Memory {
            memory: input
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>(),
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
            relative_base: 0,
            instruction_pointer: 0,
        };
        m.memory.append(&mut extramem);
        m
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub sender: Sender<i64>,
    pub receiver: Receiver<i64>,
}

impl Channel {
    fn new(bound: Option<usize>) -> Channel {
        let (sender, receiver) = match bound {
            Some(b) => bounded(b),
            None => unbounded(),
        };
        Channel { sender, receiver }
    }

    pub fn send(&self, val: i64) {
        self.sender.send(val).unwrap()
    }

    pub fn recv(&self) -> i64 {
        self.receiver.recv().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub iomode: IOMode,
    pub log_prefix: String,
    pub enable_logger: bool,
    pub mem: Memory,
    pub input_channel: Channel,
    pub output_channel: Channel,
}

impl Computer {
    pub fn from_string(input: &str, iomode: IOMode) -> Computer {
        Computer {
            mem: Memory::from_string(input),
            iomode: iomode,
            log_prefix: "".to_string(),
            enable_logger: false,
            input_channel: Channel::new(Some(1)),
            output_channel: Channel::new(None),
        }
    }

    pub fn run(&mut self) {
        loop {
            let op = Operation::from_computer(self);
            let length = Operation::get_length(&op.op_code);
            let orig_ip = self.mem.instruction_pointer;

            if self.enable_logger {
                //todo: log nicely
                self.log(format!("{:?}\tOutBuf:[{:?}]", op, self.mem.output_buffer));
            }

            match op.op_code {
                OpCode::Add => self.add(op),
                OpCode::Mul => self.mul(op),
                OpCode::Input => self.input(op),
                OpCode::Output => self.output(op),
                OpCode::JumpIfTrue => self.jump_if_true(op),
                OpCode::JumpIfFalse => self.jump_if_false(op),
                OpCode::Lessthan => self.less_than(op),
                OpCode::Equals => self.equals(op),
                OpCode::OffsetBase => self.offset_base(op),
                OpCode::End => {
                    break;
                }
            };
            if self.mem.instruction_pointer == orig_ip {
                self.increment_ip(length);
            }
        }
    }

    fn log(&mut self, str: String) {
        println!("{} -> {}", self.log_prefix, str);
    }

    fn increment_ip(&mut self, size: usize) {
        self.mem.instruction_pointer += size;
    }

    pub fn is_completed(&mut self) -> bool {
        self.mem.memory[self.mem.instruction_pointer] == 99
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
        self.mem.relative_base += self.mem.get(&op, 1);
    }

    fn input<'a>(&mut self, op: Operation) {
        let val = match self.iomode {
            IOMode::Stdio => Computer::read_stdin(),
            IOMode::Buffer => self
                .mem
                .input_buffer
                .pop_front()
                .expect("Input buffer empty"),
            IOMode::Channel => self.input_channel.receiver.recv().unwrap(),
        };
        self.mem.set(&op, 1, val);
    }

    fn output<'a>(&mut self, op: Operation) {
        let val = self.mem.get(&op, 1);
        match self.iomode {
            IOMode::Stdio => println!("Output: {}", val),
            IOMode::Buffer => self.mem.output_buffer.push_back(val),
            IOMode::Channel => self.output_channel.sender.send(val).unwrap(),
        };
    }

    fn add(&mut self, op: Operation) {
        let res = self.mem.get(&op, 1) + self.mem.get(&op, 2);
        self.mem.set(&op, 3, res);
    }

    fn mul(&mut self, op: Operation) {
        let res = self.mem.get(&op, 1) * self.mem.get(&op, 2);
        self.mem.set(&op, 3, res);
    }

    fn jump_if_true(&mut self, op: Operation) {
        match self.mem.get(&op, 1) != 0 {
            true => self.mem.instruction_pointer = self.mem.get(&op, 2) as usize,
            false => self.increment_ip(Operation::get_length(&op.op_code)),
        }
    }

    fn jump_if_false(&mut self, op: Operation) {
        match self.mem.get(&op, 1) == 0 {
            true => self.mem.instruction_pointer = self.mem.get(&op, 2) as usize,
            false => self.increment_ip(Operation::get_length(&op.op_code)),
        }
    }

    fn less_than(&mut self, op: Operation) {
        let res = self.mem.get(&op, 1) < self.mem.get(&op, 2);
        self.mem.set(&op, 3, if res == true { 1 } else { 0 });
    }

    fn equals(&mut self, op: Operation) {
        let res = self.mem.get(&op, 1) == self.mem.get(&op, 2);
        self.mem.set(&op, 3, if res == true { 1 } else { 0 });
    }
}
