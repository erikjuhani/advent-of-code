fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            s.char_indices().filter_map(|(_, c)| match c {
                '0'..='9' => c.to_digit(10),
                _ => None,
            })
        })
        .filter_map(|a| a.clone().next().zip(a.last()))
        .map(|(a, b)| a * 10 + b)
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let str_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|s| {
            s.char_indices().filter_map(|(i, c)| match c {
                '0'..='9' => c.to_digit(10),
                _ => str_digits
                    .iter()
                    .enumerate()
                    .find_map(|(n, d)| s[i..].starts_with(d).then_some((n + 1) as u32)),
            })
        })
        .filter_map(|a| a.clone().next().zip(a.last()))
        .map(|(a, b)| a * 10 + b)
        .sum::<u32>()
}

fn main() {
    let input = include_str!("../../../input/day01");
    println!("answer part1: {}", part1(input));
    println!("answer part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!(part1(test_input), 142, "for input `{:#?}`", test_input);
    }

    #[test]
    fn part_two_test() {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(part2(test_input), 281, "for input `{:#?}`", test_input);
    }
}
