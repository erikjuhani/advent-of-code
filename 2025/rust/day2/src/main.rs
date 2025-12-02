use std::ops::{ControlFlow, RangeInclusive};

fn main() {
    let ids = p1(include_str!("../../../input/day2"))
        .iter()
        .sum::<usize>();
    println!("DAY1 p1: {}", ids);
    println!(
        "DAY1 p2: {}",
        p2(include_str!("../../../input/day2"))
            .iter()
            .sum::<usize>()
    );
}

fn p1(input: &str) -> Vec<usize> {
    let ranges: Vec<RangeInclusive<usize>> = input
        .trim_end()
        .split(',')
        .flat_map(|line| line.split_once('-'))
        .map(|(first, last)| first.parse::<usize>().unwrap()..=last.parse::<usize>().unwrap())
        .collect();

    let mut ids = vec![];
    for range in ranges {
        for id in range {
            let id = id.to_string();
            let digits = id.chars();
            let mut acc = String::default();

            for digit in digits {
                acc.push(digit);
                if acc == id[acc.len()..] {
                    ids.push(id.parse::<usize>().unwrap());
                }
            }
        }
    }

    ids
}

fn p2(input: &str) -> Vec<usize> {
    let ranges: Vec<RangeInclusive<usize>> = input
        .trim_end()
        .split(',')
        .flat_map(|line| line.split_once('-'))
        .map(|(first, last)| first.parse::<usize>().unwrap()..=last.parse::<usize>().unwrap())
        .collect();

    let mut ids = vec![];
    for range in ranges {
        for id in range {
            let id = id.to_string();
            let digits = id.chars();
            let mut acc = String::default();

            for digit in digits {
                acc.push(digit);
                let x = id
                    .as_bytes()
                    .chunks(acc.len())
                    .try_fold(0, |count, digits| {
                        if acc.as_bytes() == digits {
                            ControlFlow::Continue(count + 1)
                        } else {
                            ControlFlow::Break(count)
                        }
                    });
                if x.continue_value().is_some_and(|c| c > 1) {
                    ids.push(id.parse::<usize>().unwrap());
                    break;
                }
            }
        }
    }

    ids
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    #[test]
    fn p1_partial_test() {
        let tests = [
            ("11-22", vec![11, 22]),
            ("95-115", vec![99]),
            ("998-1012", vec![1010]),
            ("1188511880-1188511890", vec![1188511885]),
        ];

        tests.into_iter().for_each(|(input, expected)| {
            assert_eq!(expected, p1(input));
        });
    }

    #[test]
    fn p1_test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(1227775554, p1(input).iter().sum::<usize>())
    }

    #[test]
    fn p2_partial_test() {
        let tests = [
            ("11-22", vec![11, 22]),
            ("95-115", vec![99, 111]),
            ("998-1012", vec![999, 1010]),
            ("1188511880-1188511890", vec![1188511885]),
            ("222220-222224", vec![222222]),
            ("1698522-1698528", vec![]),
            ("446443-446449", vec![446446]),
            ("565653-565659", vec![565656]),
        ];

        tests.into_iter().for_each(|(input, expected)| {
            assert_eq!(expected, p2(input));
        });
    }

    #[test]
    fn p2_test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(4174379265, p2(input).iter().sum::<usize>())
    }
}
