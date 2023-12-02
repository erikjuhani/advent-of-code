fn solve(input: &str, f: fn((u32, u32, u32, u32)) -> u32) -> u32 {
    input
        .lines()
        .map(|s| {
            s.split([':', ';', ',']).fold((0, 0, 0, 0), |acc, s| {
                let n = s
                    .chars()
                    .filter_map(|c| c.is_digit(10).then_some(c))
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                if s.ends_with("red") {
                    return (acc.0, acc.1.max(n), acc.2, acc.3);
                } else if s.ends_with("green") {
                    return (acc.0, acc.1, acc.2.max(n), acc.3);
                } else if s.ends_with("blue") {
                    return (acc.0, acc.1, acc.2, acc.3.max(n));
                } else {
                    return (n, acc.1, acc.2, acc.3);
                }
            })
        })
        .map(f)
        .sum()
}

fn main() {
    let input = include_str!("../../../input/day02");
    println!(
        "answer part1: {}",
        solve(input, |(id, r, g, b)| if r <= 12 && g <= 13 && b <= 14 {
            id
        } else {
            0
        })
    );
    println!("answer part2: {}", solve(input, |(_, r, g, b)| r * g * b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(
            solve(test_input, |(id, r, g, b)| {
                if r <= 12 && g <= 13 && b <= 14 {
                    id
                } else {
                    0
                }
            }),
            8,
            "for input `{}`",
            test_input
        );
    }

    #[test]
    fn part_two_test() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        assert_eq!(
            solve(test_input, |(_, r, g, b)| r * g * b),
            2286,
            "for input `{:#?}`",
            test_input
        );
    }
}
