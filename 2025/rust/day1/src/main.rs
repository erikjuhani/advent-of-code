// 0 -> 99
//
// L left
// R right

// if dial 11
// R8 -> 19
// L19 -> 0
//
// Going over 0 points to 99
// Going over 99 points to 0
//
// if dial 5
// L10 -> 95
// R5 -> 0
//
// Dial starts at 50
//
// Password is how many times dial was at 0

use std::ops::Range;

fn main() {
    println!("DAY1 p1: {}", day1p1(include_str!("../../../input/day1")));
}

fn day1p1(input: &str) -> usize {
    let dial_boundary: Range<i16> = 0..99;

    input
        .lines()
        .fold((50, 0), |(dial, count), instruction| {
            let (direction, amount) = instruction.split_at(1);
            // SAFETY: We trust that the data is valid unsigned integer
            let amount = amount.parse::<i16>().unwrap();
            let new_dial = match direction {
                "L" => {
                    let mut new_dial = dial - amount;
                    while new_dial < dial_boundary.start {
                        new_dial += dial_boundary.end + 1;
                    }
                    new_dial
                }
                // "R"
                _ => {
                    let mut new_dial = dial + amount;
                    while new_dial > dial_boundary.end {
                        new_dial -= dial_boundary.end + 1;
                    }
                    new_dial
                }
            };

            eprintln!("{}, {}", instruction, new_dial);

            (new_dial, if new_dial == 0 { count + 1 } else { count })
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1p1_test() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
L230
";

        assert_eq!(3, day1p1(input))
    }
}
