fn main() {
    println!("DAY6 p1: {}", p1(include_str!("../../../input/day6")));
    // println!("DAY6 p2: {}", p2(include_str!("../../../input/day5")));
}

fn p1(input: &str) -> usize {
    let mut worksheet = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let operators = worksheet.pop().unwrap();

    let mut results = vec![];

    for (col, operator) in operators.into_iter().enumerate() {
        let mut numbers = vec![];
        for row in &worksheet {
            numbers.push(row[col].parse::<usize>().unwrap());
        }

        match operator {
            "*" => results.push(numbers.iter().product::<usize>()),
            _ => results.push(numbers.iter().sum()),
        }
    }

    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::p1;

    #[test]
    fn p1_test() {
        let input = "123 328  51 64
45 64  387 23
 6 98  215 314
*   +   *   +";
        assert_eq!(4277556, p1(input))
    }
}
