use std::fs::read_to_string;

fn parse(input: &str) -> Option<Vec<usize>> {
    input.lines().map(|s| s.parse().ok()).collect()
}

fn main() {
    let input = read_to_string("input").expect("failed to read input file");
    let masses = parse(&input).expect("failed to parse input file");
    let p1 = pt1(&masses);
    let p2 = pt2(&masses);
    println!("Part 1 Solution: {}", p1);
    println!("Part 2 Solution: {}", p2);
}

fn pt1(masses: &Vec<usize>) -> usize {
    masses.iter().map(initial_fuel).sum()
}

fn pt2(masses: &Vec<usize>) -> usize {
    masses.iter().map(total_fuel).sum()
}

fn initial_fuel(mass: &usize) -> usize {
    return (mass / 3).checked_sub(2).unwrap_or(0);
}

fn total_fuel(mass: &usize) -> usize {
    let mut fuel = initial_fuel(mass);
    let mut fuel_to_add = 0 + fuel;
    while initial_fuel(&fuel) > 0 {
        fuel = initial_fuel(&fuel);
        fuel_to_add += fuel;
    }
    return fuel_to_add;
}

#[cfg(test)]
mod initial_fuel {
    use super::initial_fuel;

    #[test]
    fn example_1() {
        assert_eq!(initial_fuel(&12), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(initial_fuel(&14), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(initial_fuel(&1969), 654);
    }

    #[test]
    fn example_4() {
        assert_eq!(initial_fuel(&100756), 33583);
    }

    #[test]
    fn small_mass() {
        assert_eq!(initial_fuel(&2), 0);
    }
}

#[cfg(test)]
mod total_fuel {
    use super::total_fuel;

    #[test]
    fn example_1() {
        assert_eq!(total_fuel(&14), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(total_fuel(&1969), 966);
    }

    #[test]
    fn example_3() {
        assert_eq!(total_fuel(&100756), 50346);
    }
}
