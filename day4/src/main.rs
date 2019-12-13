fn main() {
    let start = 272091;
    let end = 815432;
    println!(
        "Part 1 Solution: {:?}",
        (start..end)
            .map(|n| n.to_string())
            .filter(has_incrementing_digits)
            .filter(has_2_adjacent_digits)
            .count()
    );

    println!(
        "Part 2 Solution: {:?}",
        (start..end)
            .map(|n| n.to_string())
            .filter(has_incrementing_digits)
            .filter(has_2_adjacent_advanced)
            .count()
    );
}

fn has_2_adjacent_digits(pass: &String) -> bool {
    (0..pass.len() - 1).any(|i| pass.as_bytes()[i] == pass.as_bytes()[i + 1])
}

fn has_2_adjacent_advanced(pass: &String) -> bool {
    let ba = pass.as_bytes();
    let mut counts = vec![1];
    let mut counter = 1;
    let mut current_char = ba[0];

    for i in 0..pass.len() - 1 {
        if ba[i + 1] == current_char {
            counter = counter + 1;
        } else {
            counts.push(counter);
            current_char = ba[i + 1];
            counter = 1;
        }
    }
    counts.push(counter);
    counts.iter().any(|&x| x == 2)
}

fn has_incrementing_digits(pass: &String) -> bool {
    let digits = pass
        .as_bytes()
        .into_iter()
        .map(|b| *b as i32)
        .collect::<Vec<i32>>();

    !(0..pass.len() - 1).any(|i| digits[i + 1] < digits[i])
}

#[cfg(test)]
mod has_2_adjacent_advanced {
    use super::has_2_adjacent_advanced;
    #[test]
    fn should_be_true_when_has_two_adjacent_digits() {
        assert_eq!(has_2_adjacent_advanced(&"11".to_string()), true);
        assert_eq!(has_2_adjacent_advanced(&"211".to_string()), true);
        assert_eq!(has_2_adjacent_advanced(&"112".to_string()), true);
        assert_eq!(has_2_adjacent_advanced(&"2112".to_string()), true);
    }

    #[test]
    fn should_be_true_when_only_has_three_adjacent_digits() {
        assert_eq!(has_2_adjacent_advanced(&"111".to_string()), false);
    }

    #[test]
    fn should_be_true_when_only_two_and_three_adjacent_digits() {
        assert_eq!(has_2_adjacent_advanced(&"11122".to_string()), true);
    }
}

#[cfg(test)]
mod has_2_adjacent_digits {
    use super::has_2_adjacent_digits;

    #[test]
    fn should_be_true_when_has_two_adjacent_digits() {
        assert_eq!(has_2_adjacent_digits(&"11".to_string()), true);
        assert_eq!(has_2_adjacent_digits(&"211".to_string()), true);
        assert_eq!(has_2_adjacent_digits(&"112".to_string()), true);
        assert_eq!(has_2_adjacent_digits(&"2112".to_string()), true);
    }

    #[test]
    fn should_be_false_when_doesnt_have_two_adjacent_digits() {
        assert_eq!(has_2_adjacent_digits(&"12".to_string()), false);
        assert_eq!(has_2_adjacent_digits(&"121".to_string()), false);
    }

}

#[cfg(test)]
mod has_incrementing_digits {
    use super::has_incrementing_digits;

    #[test]
    fn should_be_true_when_has_increasing_digits() {
        assert_eq!(has_incrementing_digits(&"11".to_string()), true);
        assert_eq!(has_incrementing_digits(&"11".to_string()), true);
        assert_eq!(has_incrementing_digits(&"112".to_string()), true);
        assert_eq!(has_incrementing_digits(&"1112349".to_string()), true);
    }

    #[test]
    fn should_be_false_when_doesnt_have_all_increasing_digits() {
        assert_eq!(has_incrementing_digits(&"21".to_string()), false);
        assert_eq!(has_incrementing_digits(&"121".to_string()), false);
        assert_eq!(has_incrementing_digits(&"223450".to_string()), false);
    }
}
