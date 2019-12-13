use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").expect("failed to read input file");

    let result = run(&input[..], 12, 2);

    println!("Part 1 Solution: {}", result[0]);

    'outer: for n in 1..168 {
        for v in 1..168 {
            let result = run(&input[..], n, v)[0];
            if result == 19690720 {
                println!("Part 2 Solution: {}", 100 * n + v);
                break 'outer;
            }
        }
    }
}

fn run(input: &str, noun: usize, verb: usize) -> Vec<usize> {
    let mut memory = parse(&input).expect("failed to parse input");
    let mut instruction_ptr = 0;

    memory[1] = noun;
    memory[2] = verb;

    loop {
        let opcode = memory[instruction_ptr];
        match opcode {
            1 => memory = add(&mut memory, instruction_ptr).to_vec(),
            2 => memory = mul(&mut memory, instruction_ptr).to_vec(),
            _ => {
                break;
            }
        };
        instruction_ptr = instruction_ptr + 4;
    }

    return memory;
}

fn add<'a>(memory: &'a mut [usize], instruction_ptr: usize) -> &'a mut [usize] {
    let operation: &[usize] = &memory[instruction_ptr..instruction_ptr + 4];
    memory[operation[3]] = memory[operation[1]] + memory[operation[2]];
    return memory;
}

fn mul<'a>(memory: &'a mut [usize], instruction_ptr: usize) -> &'a mut [usize] {
    let operation: &[usize] = &memory[instruction_ptr..instruction_ptr + 4];
    memory[operation[3]] = memory[operation[1]] * memory[operation[2]];
    return memory;
}

fn parse(input: &str) -> Option<Vec<usize>> {
    return input.split(",").map(|s| s.parse().ok()).collect();
}

#[cfg(test)]
mod examples {
    use super::run;

    #[test]
    fn example_1() {
        let input = "1,0,0,0,99";
        assert_eq!(run(input, 0, 0), [2, 0, 0, 0, 99])
    }

    #[test]
    fn example_2() {
        let input = "2,3,0,3,99";
        assert_eq!(run(input, 3, 0), [2, 3, 0, 6, 99])
    }

    #[test]
    fn example_3() {
        let input = "2,4,4,5,99,0";
        assert_eq!(run(input, 4, 4), [2, 4, 4, 5, 99, 9801])
    }

    #[test]
    fn example_4() {
        let input = "1,1,1,4,99,5,6,0,99";
        assert_eq!(run(input, 1, 1), [30, 1, 1, 4, 2, 5, 6, 0, 99])
    }
}
