mod computer {
    use computer::{Computer, IOMode};

    fn run_input(input: &str) -> Computer {
        let mut c = Computer::from_string(input, IOMode::Stdio);
        c.run();
        c
    }

    #[test]
    fn input_buffer() {
        let mut c = Computer::from_string("3,3,99,0", IOMode::Buffer);
        c.input_to_buffer(12);
        c.run();
        assert_eq!(c.mem[0..4], [3, 3, 99, 12]);
    }

    #[test]
    fn input_buffer_rel_1() {
        let mut c = Computer::from_string("203,2,99,0", IOMode::Buffer);
        c.input_to_buffer(12);
        c.enable_logger = true;
        c.run();
        assert_eq!(c.mem[0..4], [203, 2, 99, 12]);
        println!("{:?}", c.mem[0..6].to_vec());
    }

    #[test]
    fn input_buffer_rel_2() {
        let mut c = Computer::from_string("203,-1,99", IOMode::Buffer);
        c.input_to_buffer(12);
        c.enable_logger = true;
        c.run();
        assert_eq!(c.mem[0..3], [12, -1, 99]);
        println!("{:?}", c.mem[0..6].to_vec());
    }

    #[test]
    fn input_buffer_rel_3() {
        let mut c = Computer::from_string("109,2,203,0,99,0", IOMode::Buffer);
        c.input_to_buffer(12);
        c.enable_logger = true;
        c.run();
        assert_eq!(c.mem[0..6], [109, 2, 203, 0, 99, 12]);
        println!("{:?}", c.mem[0..6].to_vec());
    }

    #[test]
    fn input_buffer_rel_4() {
        let mut c = Computer::from_string("109,-2,203,0,99", IOMode::Buffer);
        c.input_to_buffer(12);
        c.enable_logger = true;
        c.run();
        assert_eq!(c.mem[0..5], [109, 12, 203, 0, 99]);
        println!("{:?}", c.mem[0..5].to_vec());
    }

    #[test]
    fn offset_multiple() {
        let mut c = Computer::from_string("109,998,209,12,9,1000", IOMode::Buffer);
        c.enable_logger = true;
        c.run();
        assert_eq!(c.mem[0..5], [109, 12, 203, 0, 99]);
        println!("{:?}", c.mem[0..5].to_vec());
    }

    #[test]
    #[should_panic(expected = "Timed out waiting for input buffer")]
    fn input_buffer_error_when_empty() {
        let mut c = Computer::from_string("3,3,99,0", IOMode::Buffer);
        c.run();
    }

    #[test]
    fn offset_relative_base() {
        let c = run_input("109,3,99");
        assert_eq!(c.relative_base, 3);

        let c = run_input("209,2,99,123");
        assert_eq!(c.relative_base, 123);

        let c = run_input("209,2,99,-4");
        assert_eq!(c.relative_base, -4);
    }

    #[test]
    fn output_buffer() {
        let mut c = Computer::from_string("4,3,99,12", IOMode::Buffer);
        c.run();
        let output = c.output_from_buffer();
        assert_eq!(output, 12);
    }

    #[test]
    #[should_panic(expected = "Timed out waiting for output buffer")]
    fn output_buffer_error_when_empty() {
        let mut c = Computer::from_string("99", IOMode::Buffer);
        c.run();
        c.output_from_buffer();
    }

    #[test]
    fn add_pos() {
        let c = run_input("1,0,0,0,99");
        assert_eq!(c.mem[0..5], [2, 0, 0, 0, 99])
    }

    #[test]
    fn mul_pos() {
        let c = run_input("2,3,0,3,99");
        assert_eq!(c.mem[0..5], [2, 3, 0, 6, 99])
    }

    #[test]
    fn day2_example_4() {
        let c = run_input("1,1,1,4,99,5,6,0,99");
        assert_eq!(c.mem[0..9], [30, 1, 1, 4, 2, 5, 6, 0, 99])
    }

    #[test]
    fn add_imm() {
        let c = run_input("1101,2,3,0,99");
        assert_eq!(c.mem[0..5], [5, 2, 3, 0, 99])
    }

    #[test]
    fn mul_imm() {
        let c = run_input("1102,2,3,0,99");
        assert_eq!(c.mem[0..5], [6, 2, 3, 0, 99])
    }

    #[test]
    fn jit_pos() {
        // if mem[5] != 0, jmp 4 + end
        let mut c = run_input("5,5,6,99,99,1,4");
        assert_eq!(c.ip, 4);

        c = run_input("5,5,6,99,99,0,4");
        assert_eq!(c.ip, 3)
    }

    #[test]
    fn jit_imm() {
        let mut c = run_input("1105,1,4,99,99");
        assert_eq!(c.ip, 4);

        c = run_input("1105,0,4,99,99");
        assert_eq!(c.ip, 3);
    }

    #[test]
    fn jif_pos() {
        // if mem[5] != 0, jmp 4 + end
        let mut c = run_input("6,5,6,99,99,1,4");
        assert_eq!(c.ip, 3);

        c = run_input("6,5,6,99,99,0,4");
        assert_eq!(c.ip, 4)
    }

    #[test]
    fn jif_imm() {
        let mut c = run_input("1106,1,4,99,99");
        assert_eq!(c.ip, 3);

        c = run_input("1106,0,4,99,99");
        assert_eq!(c.ip, 4);
    }

    #[test]
    fn lessthan_pos() {
        let mut c = run_input("7,5,6,7,99,3,4,0");
        assert_eq!(c.mem[0..8], [7, 5, 6, 7, 99, 3, 4, 1]);

        c = run_input("7,5,6,7,99,4,3,0");
        assert_eq!(c.mem[0..8], [7, 5, 6, 7, 99, 4, 3, 0]);
    }

    #[test]
    fn lessthan_imm() {
        let mut c = run_input("1107,3,4,5,99,0");
        assert_eq!(c.mem[0..6], [1107, 3, 4, 5, 99, 1]);

        c = run_input("1107,4,3,5,99,0");
        assert_eq!(c.mem[0..6], [1107, 4, 3, 5, 99, 0]);
    }

    #[test]
    fn eq_pos() {
        let mut c = run_input("8,5,6,7,99,4,4,0");
        assert_eq!(c.mem[0..8], [8, 5, 6, 7, 99, 4, 4, 1]);

        c = run_input("8,5,6,7,99,4,3,0");
        assert_eq!(c.mem[0..8], [8, 5, 6, 7, 99, 4, 3, 0]);
    }

    #[test]
    fn eq_imm() {
        let mut c = run_input("1108,4,4,5,99,0");
        assert_eq!(c.mem[0..6], [1108, 4, 4, 5, 99, 1]);

        c = run_input("1108,4,5,5,99,0");
        assert_eq!(c.mem[0..6], [1108, 4, 5, 5, 99, 0]);
    }
}
