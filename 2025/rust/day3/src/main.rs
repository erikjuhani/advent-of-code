fn main() {
    println!("DAY1 p1: {}", p1(include_str!("../../../input/day3")));
}

// One line of digits = One battery bank
//
// Turn on exactly 2 batteries
// Joltage produced = number formed by digits on turned on batteries
//
// Bank: 12345, turn on 2 and 4
// Result: 2 + 4 = 24 jolts
//
// Find largest possible joltage each bank can produce
fn p1(input: &str) -> u32 {
    // cursor location in bank start at idx 0, increment after loop through chars
    // ^ skip location
    // first_char -> loop over the digits
    // put digits together
    // make into number
    // store number in variable
    // compare if number is bigger
    // compare first char and window

    input
        .lines()
        .map(|line| {
            let mut max_joltage = 0;

            for (cursor, a) in line.char_indices() {
                for b in line[cursor + 1..].chars() {
                    let n = format!("{a}{b}").parse::<u32>().unwrap();
                    if max_joltage < n {
                        max_joltage = n;
                    }
                }
            }

            max_joltage
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::p1;

    #[test]
    fn p1_parts() {
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
}
