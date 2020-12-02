use std::cmp::min;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| s.parse::<String>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part_1(input: &[String]) -> i32 {
    let mut correct_passwords = 0;
    for line in input {
        let mut segments = line.split_whitespace();
        let mut policy = segments.next().unwrap().split("-");
        let minimum: usize = policy.next().unwrap().parse().unwrap();
        let maximum: usize = policy.next().unwrap().parse().unwrap();

        let character = segments
            .next()
            .unwrap()
            .replace(':', "")
            .chars()
            .next()
            .unwrap();
        let password = segments.next().unwrap();

        let current_count = password.chars().filter(|c| c == &character).count();
        // why the fuck doesn't it have the python syntax for this, ugh
        if maximum >= current_count && current_count >= minimum {
            correct_passwords += 1
        }
    }
    correct_passwords
}

#[cfg(test)]
mod tests {
    use super::part_1;

    #[test]
    fn test_part1() {
        let input = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];
        assert_eq!(2, part_1(&input));
    }
}
