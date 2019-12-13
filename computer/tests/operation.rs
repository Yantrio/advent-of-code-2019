mod operation {
    use computer::{Computer, IOMode, Mode, OpCode, Operation};

    #[test]
    fn should_parse_opcode_with_missing_initial_zero() {
        let c = Computer::from_string("1002,3,3,0", IOMode::Stdio);

        let opcode = Operation::from_computer(&c);
        assert_eq!(
            opcode,
            Operation {
                op_code: OpCode::Mul,
                data: vec![1002, 3, 3, 0],
                modes: (Mode::Position, Mode::Immediate, Mode::Position)
            }
        )
    }

    #[test]
    fn should_parse_opcode_1() {
        let c = Computer::from_string("01001,100,1,100", IOMode::Stdio);
        let opcode = Operation::from_computer(&c);
        assert_eq!(
            opcode,
            Operation {
                op_code: OpCode::Add,
                data: vec![01001, 100, 1, 100],
                modes: (Mode::Position, Mode::Immediate, Mode::Position)
            }
        );
    }

    #[test]
    fn should_parse_opcode() {
        let c = Computer::from_string("01002,3,3,0", IOMode::Stdio);
        let opcode = Operation::from_computer(&c);
        assert_eq!(
            opcode,
            Operation {
                op_code: OpCode::Mul,
                data: vec![1002, 3, 3, 0],
                modes: (Mode::Position, Mode::Immediate, Mode::Position)
            }
        );

        let c = Computer::from_string("01002,3,3,0", IOMode::Stdio);
        let opcode = Operation::from_computer(&c);
        assert_eq!(
            opcode,
            Operation {
                op_code: OpCode::Mul,
                data: vec![1002, 3, 3, 0],
                modes: (Mode::Position, Mode::Immediate, Mode::Position)
            }
        )
    }
}
