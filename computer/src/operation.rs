use super::Computer;
use num_enum::TryFromPrimitive;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    pub fn from_i64(input: i64) -> Mode {
        match input {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown parameter mode {}", input),
        }
    }
}

#[derive(Debug, TryFromPrimitive, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    Lessthan = 7,
    Equals = 8,
    OffsetBase = 9,
    End = 99,
}

type Modes = (Mode, Mode, Mode);

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub op_code: OpCode,
    pub modes: Modes,
    pub data: Vec<i64>,
}

impl Operation {
    pub fn get_length(opcode: &OpCode) -> usize {
        lazy_static! {
            static ref HASHMAP: HashMap<OpCode, usize> = {
                let mut m = HashMap::new();
                m.insert(OpCode::Add, 4);
                m.insert(OpCode::Mul, 4);
                m.insert(OpCode::Input, 2);
                m.insert(OpCode::Output, 2);
                m.insert(OpCode::JumpIfTrue, 3);
                m.insert(OpCode::JumpIfFalse, 3);
                m.insert(OpCode::Lessthan, 4);
                m.insert(OpCode::Equals, 4);
                m.insert(OpCode::End, 1);
                m.insert(OpCode::OffsetBase, 2);
                m
            };
        }
        *HASHMAP.get(opcode).unwrap()
    }

    pub fn from_computer(computer: &Computer) -> Operation {
        let raw_opcode = computer.mem.memory[computer.mem.instruction_pointer];
        let opcode = OpCode::try_from((raw_opcode % 100) as i32).expect("Failed to parse opcode");
        let length = Operation::get_length(&opcode);
        let data = computer.mem.memory
            [computer.mem.instruction_pointer..computer.mem.instruction_pointer + length]
            .to_vec();
        Operation {
            op_code: opcode,
            modes: (
                Mode::from_i64(raw_opcode / 100 % 10),
                Mode::from_i64(raw_opcode / 1000 % 10),
                Mode::from_i64(raw_opcode / 10000 % 10),
            ),
            data,
        }
    }
}
