fn main() {
    println!("DAY6 p1: {}", p1(include_str!("../../../input/day6")));
    println!("DAY6 p2: {}", p2(include_str!("../../../input/day6")));
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

fn p2(input: &str) -> usize {
    let op_vec = input.lines().last().unwrap();
    let mut op_iter = op_vec.char_indices().rev().peekable();

    let mut res = vec![];
    while let Some((i, op)) = op_iter.next() {
        match op {
            '+' | '*' => {
                let mut size = 0;
                while let Some((_, ' ')) = op_iter.peek() {
                    op_iter.next();
                    size += 1;
                }
                if op_iter.peek().is_none() {
                    size += 1;
                }
                res.push((i, size, op))
            }
            _ => {
                op_iter.next();
            }
        }
    }

    let mut worksheet = input
        .lines()
        .take(input.lines().count() - 1)
        .map(|line| line.chars().collect::<Vec<_>>())
        // .map(|line| {
        //     res.iter()
        //         .map(
        //             |&(idx, size, _)| {
        //                 line.chars().skip(idx).take(size + 1).collect::<Vec<_>>()
        //                 // line.as_bytes()
        //                 //     .iter()
        //                 //     .skip(*idx)
        //                 //     .take(*size + 2)
        //                 //     .enumerate()
        //                 //     .fold(vec![' '; *size], |mut acc, (i, b)| {
        //                 //         eprintln!("{:?}, {:?}", i, char::from(*b));
        //                 //         acc[2 - i] = char::from(*b);
        //                 //         acc
        //                 //     })
        //             }, /* line.chars().skip(*idx).take(*size + 1).collect::<Vec<_>>() */
        //         )
        //         .collect::<Vec<_>>()
        // })
        .collect::<Vec<_>>();

    eprintln!("{:?}", worksheet);
    for (idx, size, op) in res.iter() {
        for i in *idx..idx + *size {
            for row in &worksheet {
                eprintln!("{:?}", row.get(*idx..idx + *size));
            }
        }
    }
    // eprintln!("{:?}", res);

    // let mut result = vec![];
    // for (idx, size, op) in res.iter() {
    //     let mut numbers = vec![];
    //     for i in *idx..idx + *size {
    //         let mut number = String::new();
    //         for row in &worksheet {
    //             match row.get(i) {
    //                 Some(' ') => {}
    //                 Some(_) => number.push(row[i]),
    //                 None => {}
    //             }
    //         }
    //         eprintln!("{op} {number}");
    //         numbers.push(number.parse::<usize>().unwrap());
    //     }
    //
    //     match op {
    //         '*' => {
    //             result.push(numbers.iter().product::<usize>());
    //             numbers.clear();
    //         }
    //         '+' => {
    //             result.push(numbers.iter().sum::<usize>());
    //             numbers.clear();
    //         }
    //         _ => {}
    //     }
    // }
    //
    // eprintln!("{:?}", result);

    // eprintln!("{:?}", worksheet);

    // let mut worksheet = input
    //     .lines()
    //     .map(|line| {
    //         line.as_bytes()
    //             .rev()
    //             .flat_map(|b| {
    //                 b.iter()
    //                     .take(3)
    //                     .enumerate()
    //                     .fold(vec![' '; 3], |mut acc, (i, b)| {
    //                         acc[2 - i] = char::from(*b);
    //                         acc
    //                     })
    //             })
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();

    0
    // let mut numbers = vec![];
    // let mut result = vec![];
    // for (col, operator) in operators.iter().enumerate() {
    //     let mut number = String::new();
    //     for row in worksheet.iter() {
    //         match row[col] {
    //             ' ' => {}
    //             _ => number.push(row[col]),
    //         }
    //     }
    //     numbers.push(number.parse::<usize>().unwrap());
    //
    //     eprintln!("{operator} {number} {:?}", numbers);
    //
    //     match operator {
    //         '*' => {
    //             result.push(numbers.iter().product::<usize>());
    //             numbers.clear();
    //         }
    //         '+' => {
    //             result.push(numbers.iter().sum::<usize>());
    //             numbers.clear();
    //         }
    //         _ => {}
    //     }
    // }
    //
    // result.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        assert_eq!(4277556, p1(input))
    }

    #[test]
    fn p2_test() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!(3263827, p2(input))
    }
}
