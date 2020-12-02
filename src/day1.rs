#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part_1(input: &[i32]) -> i32 {
    for a in input {
        for b in input {
            if a + b == 2020 {
                return a * b
            }
        }
    }
    unimplemented!()
}

#[aoc(day1, part2)]
pub fn part_2(input: &[i32]) -> i32 {
    for a in input {
        for b in input {
            for c in input {
                if a + b + c == 2020 {
                    return a * b * c
                }
            }
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::day1::{part_1, part_2};

    #[test]
    fn test_part1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, part_1(&input));
    }

    #[test]
    fn test_part2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(241861950, part_2(&input));
    }
}