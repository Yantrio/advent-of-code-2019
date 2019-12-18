mod computer {
    use computer::{Computer, IOMode};

    fn run_input(input: &str) -> Computer {
        let mut c = Computer::from_string(input, IOMode::Stdio);
        c.run();
        c
    }

    #[test]
    fn input_channel() {
        let mut c = Computer::from_string("3,3,99,0", IOMode::Channel);
        c.input_channel.send(12);
        c.run();
        assert_eq!(c.mem.memory[0..4], [3, 3, 99, 12]);
    }

    #[test]
    fn output_channel() {
        let mut c = Computer::from_string("4,3,99,10", IOMode::Channel);
        c.run();
        let res = c.output_channel.recv();
        assert_eq!(res, 10);
    }
    #[test]
    fn input_buffer() {
        let mut c = Computer::from_string("3,3,99,0", IOMode::Buffer);
        c.mem.input_buffer.push_back(12);
        c.run();
        assert_eq!(c.mem.memory[0..4], [3, 3, 99, 12]);
    }

    #[test]
    fn input_buffer_rel_1() {
        let mut c = Computer::from_string("203,3,99,0", IOMode::Buffer);
        c.mem.input_buffer.push_back(12);
        c.enable_logger = true;
        c.run();
        assert_eq!(c.mem.memory[0..4], [203, 3, 99, 12]);
    }

    #[test]
    fn input_buffer_rel_2() {
        let mut c = Computer::from_string("109,2,203,0,99", IOMode::Buffer);
        c.mem.input_buffer.push_back(12);
        c.enable_logger = true;
        c.run();
        assert_eq!(c.mem.memory[0..5], [109, 2, 12, 0, 99]);
    }

    #[test]
    fn offset_relative_base() {
        let c = run_input("109,3,99");
        assert_eq!(c.mem.relative_base, 3);

        let c = run_input("209,3,99,123");
        assert_eq!(c.mem.relative_base, 123);

        let c = run_input("209,3,99,-4");
        assert_eq!(c.mem.relative_base, -4);
    }

    #[test]
    fn output_buffer() {
        let mut c = Computer::from_string("4,3,99,12", IOMode::Buffer);
        c.run();
        let output = c.mem.output_buffer.pop_front().unwrap();
        assert_eq!(output, 12);
    }

    #[test]
    fn add_pos() {
        let c = run_input("1,0,0,0,99");
        assert_eq!(c.mem.memory[0..5], [2, 0, 0, 0, 99])
    }

    #[test]
    fn mul_pos() {
        let c = run_input("2,3,0,3,99");
        assert_eq!(c.mem.memory[0..5], [2, 3, 0, 6, 99])
    }

    #[test]
    fn day2_example_4() {
        let c = run_input("1,1,1,4,99,5,6,0,99");
        assert_eq!(c.mem.memory[0..9], [30, 1, 1, 4, 2, 5, 6, 0, 99])
    }

    #[test]
    fn add_imm() {
        let c = run_input("1101,2,3,0,99");
        assert_eq!(c.mem.memory[0..5], [5, 2, 3, 0, 99])
    }

    #[test]
    fn mul_imm() {
        let c = run_input("1102,2,3,0,99");
        assert_eq!(c.mem.memory[0..5], [6, 2, 3, 0, 99])
    }

    #[test]
    fn jit_pos() {
        // if mem[5] != 0, jmp 4 + end
        let mut c = run_input("5,5,6,99,99,1,4");
        assert_eq!(c.mem.instruction_pointer, 4);

        c = run_input("5,5,6,99,99,0,4");
        assert_eq!(c.mem.instruction_pointer, 3)
    }

    #[test]
    fn jit_imm() {
        let mut c = run_input("1105,1,4,99,99");
        assert_eq!(c.mem.instruction_pointer, 4);

        c = run_input("1105,0,4,99,99");
        assert_eq!(c.mem.instruction_pointer, 3);
    }

    #[test]
    fn jif_pos() {
        // if mem[5] != 0, jmp 4 + end
        let mut c = run_input("6,5,6,99,99,1,4");
        assert_eq!(c.mem.instruction_pointer, 3);

        c = run_input("6,5,6,99,99,0,4");
        assert_eq!(c.mem.instruction_pointer, 4)
    }

    #[test]
    fn jif_imm() {
        let mut c = run_input("1106,1,4,99,99");
        assert_eq!(c.mem.instruction_pointer, 3);

        c = run_input("1106,0,4,99,99");
        assert_eq!(c.mem.instruction_pointer, 4);
    }

    #[test]
    fn lessthan_pos() {
        let mut c = run_input("7,5,6,7,99,3,4,0");
        assert_eq!(c.mem.memory[0..8], [7, 5, 6, 7, 99, 3, 4, 1]);

        c = run_input("7,5,6,7,99,4,3,0");
        assert_eq!(c.mem.memory[0..8], [7, 5, 6, 7, 99, 4, 3, 0]);
    }

    #[test]
    fn lessthan_imm() {
        let mut c = run_input("1107,3,4,5,99,0");
        assert_eq!(c.mem.memory[0..6], [1107, 3, 4, 5, 99, 1]);

        c = run_input("1107,4,3,5,99,0");
        assert_eq!(c.mem.memory[0..6], [1107, 4, 3, 5, 99, 0]);
    }

    #[test]
    fn eq_pos() {
        let mut c = run_input("8,5,6,7,99,4,4,0");
        assert_eq!(c.mem.memory[0..8], [8, 5, 6, 7, 99, 4, 4, 1]);

        c = run_input("8,5,6,7,99,4,3,0");
        assert_eq!(c.mem.memory[0..8], [8, 5, 6, 7, 99, 4, 3, 0]);
    }

    #[test]
    fn eq_imm() {
        let mut c = run_input("1108,4,4,5,99,0");
        assert_eq!(c.mem.memory[0..6], [1108, 4, 4, 5, 99, 1]);

        c = run_input("1108,4,5,5,99,0");
        assert_eq!(c.mem.memory[0..6], [1108, 4, 5, 5, 99, 0]);
    }
}
