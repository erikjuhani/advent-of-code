use std::mem::replace;

fn main() {
    println!("DAY1 p1: {}", p1(include_str!("../../../input/day3")));
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut max_joltage = 0;

            for (cursor, a) in line.char_indices() {
                for b in line[cursor + 1..].chars() {
                    let n = format!("{a}{b}").parse::<usize>().unwrap();
                    if max_joltage < n {
                        max_joltage = n;
                    }
                }
            }

            max_joltage
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut max_joltage = 0;
            for x in (0..12) {
                for (cursor, a) in line.char_indices() {
                    for b in line[cursor + 1..].chars() {
                        let n = format!("{a}{b}").parse::<usize>().unwrap();
                        if max_joltage < n {
                            max_joltage = n;
                        }
                    }
                }
            }
            max_joltage
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    #[test]
    fn p1_test() {
        let tests = [
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
            (
                "987654321111111\n811111111111119\n234234234234278\n818181911112111",
                357,
            ),
        ];

        tests
            .into_iter()
            .for_each(|(input, expected)| assert_eq!(expected, p1(input)))
    }

    #[test]
    fn p2_test() {
        let tests = [
            ("987654321111111", 987654321111),
            ("811111111111119", 811111111119),
            ("234234234234278", 434234234278),
            ("818181911112111", 888911112111),
            (
                "987654321111111\n811111111111119\n234234234234278\n818181911112111",
                3121910778619,
            ),
        ];

        tests
            .into_iter()
            .for_each(|(input, expected)| assert_eq!(expected, p2(input)))
    }
}
