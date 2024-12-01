use std::collections::HashMap;

fn main() {
    let data = include_str!("../../../input/day1");
    let res1 = part1(data);
    let res2 = part2(data);

    println!("part1: {res1:?}");
    println!("part2: {res2:?}");
}

fn part1(data: &str) -> usize {
    let (mut a, mut b) = data
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            a.parse::<usize>().ok().zip(b.parse::<usize>().ok())
        })
        .fold((vec![], vec![]), |mut acc, (a, b)| {
            acc.0.push(a);
            acc.1.push(b);
            acc
        });

    a.sort();
    b.sort();

    a.iter()
        .enumerate()
        .filter_map(|(i, a)| b.get(i).map(|b| if a > b { a - b } else { b - a }))
        .sum::<usize>()
}

fn part2(data: &str) -> usize {
    let (mut a, b) = data
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            a.parse::<usize>().ok().zip(b.parse::<usize>().ok())
        })
        .fold((vec![], HashMap::new()), |mut acc, (a, b)| {
            acc.0.push(a);
            acc.1.entry(b).and_modify(|n| *n += 1).or_insert(1);
            acc
        });

    a.sort();

    return a
        .iter()
        .filter_map(|a| b.get(a).map(|b| *a * b))
        .sum::<usize>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(part1(test_input), 11, "for input `{:#?}`", test_input);
    }

    #[test]
    fn part_two_test() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(part2(test_input), 31, "for input `{:#?}`", test_input);
    }
}
