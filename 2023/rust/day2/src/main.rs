fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|s| {
            let g = s.split(':').collect::<Vec<&str>>();
            return g
                .first()
                .cloned()
                .map(|s| {
                    s.chars()
                        .filter_map(|c| c.is_digit(10).then_some(c))
                        .collect::<String>()
                })
                .map(|s| s.parse::<u32>().unwrap())
                .zip(
                    g[1..]
                        .iter()
                        .map(|s| {
                            s.split(';')
                                .map(|s| {
                                    s.split(',').fold((0, 0, 0), |acc, s| {
                                        let (amount, color) = s.trim().split_once(' ').unwrap();
                                        let n = amount.parse::<u32>().unwrap();
                                        match color {
                                            "red" => (acc.0 + n, acc.1, acc.2),
                                            "green" => (acc.0, acc.1 + n, acc.2),
                                            "blue" => (acc.0, acc.1, acc.2 + n),
                                            _ => acc,
                                        }
                                    })
                                })
                                .collect::<Vec<(u32, u32, u32)>>()
                        })
                        .flatten()
                        .all(|(r, g, b)| r <= 12 && g <= 13 && b <= 14)
                        .then_some(Some(true)),
                )
                .map(|(id, _)| id);
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (_, t) = s.split_once(':').unwrap();
            return t
                .split(';')
                .map(|s| {
                    s.split(',').fold((0, 0, 0), |acc, s| {
                        let (amount, color) = s.trim().split_once(' ').unwrap();
                        let n = amount.parse::<u32>().unwrap();
                        match color {
                            "red" => (n, acc.1, acc.2),
                            "green" => (acc.0, n, acc.2),
                            "blue" => (acc.0, acc.1, n),
                            _ => acc,
                        }
                    })
                })
                .collect::<Vec<_>>();
        })
        .map(|x| {
            x.iter().fold((1, 1, 1), |acc, &(r, g, b)| {
                (acc.0.max(r), acc.1.max(g), acc.2.max(b))
            })
        })
        .map(|(r, g, b)| r * g * b)
        .sum::<u32>()
}

fn main() {
    let input = include_str!("../../../input/day02");
    println!("answer part1: {}", part1(input));
    println!("answer part2: {}", part2(input));
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
        assert_eq!(part1(test_input), 8, "for input `{}`", test_input);
    }

    #[test]
    fn part_two_test() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        assert_eq!(part2(test_input), 2286, "for input `{:#?}`", test_input);
    }
}
