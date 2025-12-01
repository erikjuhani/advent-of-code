fn main() {
    let (a, b) = p(include_str!("../../../input/day1"));
    println!("DAY1 p1: {}", a);
    println!("DAY1 p2: {}", b);
}

fn p(input: &str) -> (i32, i32) {
    let boundary = 100;

    input
        .lines()
        .fold((50, (0, 0)), |(dial, (zeros, passed)), inst| {
            let (dir, amount) = inst.split_at(1);
            let amount = amount
                .parse::<i32>()
                .map(|n| if dir == "L" { -n } else { n })
                .unwrap();

            let at_zero: i32 = ((dial + amount) % boundary == 0).into();

            let delta = if amount >= 0 {
                (dial + amount) / boundary
            } else {
                // going left is tricky, calc distance to next zero crossing
                // we have to reverse the dial since the amount we calculate is actual increasing,
                // so we flip the direction so e.g. 40 turns into 60 and L50 will result in 60 + 50
                // which is 110 and goes over the boundary once.
                //
                // if we didn't do that we would end with 40 + 50 which is 90 and doesn't go over
                // the boundary.
                let offset = (boundary - dial) % boundary;
                (offset - amount) / boundary
            };

            // we want remainder, rem_euclid gets the remainder against the boundary
            let dial = (dial + amount).rem_euclid(boundary);

            (dial, (zeros + at_zero, passed + delta))
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
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
";

        assert_eq!(3, p(input).0)
    }

    #[test]
    fn p2_test_a() {
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
";

        assert_eq!(6, p(input).1)
    }

    #[test]
    fn p2_test_b() {
        let input = "R1000";

        assert_eq!(10, p(input).1)
    }
}
