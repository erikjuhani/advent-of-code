use std::ops::RangeInclusive;

fn main() {
    let ids = p1(include_str!("../../../input/day2"))
        .iter()
        .sum::<usize>();
    println!("DAY1 p1: {}", ids);
    // println!("DAY1 p2: {}", b);
}

fn p1(input: &str) -> Vec<usize> {
    // If next is same digit set as invalid digit
    // collector / accumulator
    // accumulate until we meet a same digit in the id
    //
    // create ranges
    //
    // 11-22 unwraps into 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22
    // ^ only two of them have repeating digits 11 and 22
    //
    // 95-115 unwraps into 95, 96, 97, 98, 99, 100 .. 113, 114, 115
    // ^ 99 has repeating digits
    //
    let ranges: Vec<RangeInclusive<usize>> = input
        .trim_end()
        .split(',')
        .flat_map(|line| line.split_once('-'))
        .map(|(first, last)| first.parse::<usize>().unwrap()..=last.parse::<usize>().unwrap())
        .collect();

    let mut ids = vec![];
    for range in ranges {
        // let s = 12.to_string(); // "12"
        // s.chars(); '1', '1'
        //
        // let acc = "";                          / 1
        // acc += c       / 1                     / 0
        //
        // if acc = s[acc.len()..] { // 1 == 010  / 10 == 10
        //    acc = "";
        //    ids.push(s);

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

#[cfg(test)]
mod tests {
    use crate::p1;

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
}
