use std::collections::HashSet;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|s| {
            s.split_once(": ")
                .and_then(|(_, table)| table.split_once(" | "))
                .map(|(winning_numbers, numbers)| {
                    (
                        winning_numbers
                            .split_whitespace()
                            .collect::<HashSet<&str>>(),
                        numbers.split_whitespace().collect::<HashSet<&str>>(),
                    )
                })
        })
        .map(|(n, m)| {
            n.intersection(&m)
                .map(|x| x.parse::<u32>().unwrap())
                .enumerate()
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum()
}

fn main() {
    let input = include_str!("../../../input/day04");
    println!("answer part1: {}", part1(input));
    // println!("answer part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_one_test() {
        assert_eq!(part1(TEST_INPUT), 13, "for input `{}`", TEST_INPUT);
    }

    // #[test]
    // fn part_two_test() {
    //     assert_eq!(part2(TEST_INPUT), 30, "for input `{}`", TEST_INPUT);
    // }
}
