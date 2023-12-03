use std::iter;

#[derive(Debug)]
enum Value {
    Part(usize, (usize, usize, usize)),
    Gear(usize, usize),
    Symbol(usize, usize),
}

fn check_edges(lines: Vec<&str>, (x, y, size): (usize, usize, usize)) -> bool {
    let edges: [(i32, i32); 8] = [
        (0, -1),  // N
        (1, -1),  // NE
        (1, 0),   // E
        (-1, 1),  // SE
        (0, 1),   // S
        (1, 1),   // SW
        (-1, 0),  // W
        (-1, -1), // NW
    ];

    (x..size)
        .find_map(|x| {
            edges.iter().find_map(|(edge_x, edge_y)| {
                let bounded_y = if y == 0 || y == lines.len() { y } else { 1 };

                println!("{}", (*edge_y) as usize);

                lines.get(y).and_then(|&s| {
                    println!("{}", s);
                    let bounded_x = if x == 0 || x == s.len() {
                        x
                    } else {
                        (x as i32 + *edge_x) as usize
                    };

                    s.get(bounded_x..bounded_x + 1).and_then(|s| {
                        s.chars().find_map(|c| match c {
                            c if c.is_ascii_digit() => None,
                            '.' => None,
                            c => Some(c),
                        })
                    })
                })
            })
        })
        .is_some()
}

fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    // loop chars
    // take while ascii digit
    //   -> check neighbours for other symbols than .
    //   -> if found make Some(ascii_digits) else None
    let mut tokens: Vec<Value> = vec![];

    let mut c: Vec<String> = vec![];

    for (y, s) in lines.clone().iter().enumerate() {
        let mut chars = s.char_indices().peekable();
        while let Some(&(x, c)) = chars.peek() {
            match c {
                c if c.is_ascii_digit() => {
                    let num_token = iter::from_fn(|| chars.next_if(|(_, c)| c.is_ascii_digit()))
                        .map(|(_, c)| c)
                        .collect::<String>();
                    // Check edges to see if the number is valid, if not we do not append to
                    // tokens
                    println!("{}", num_token);
                    if check_edges(lines.clone(), (x, y, num_token.len())) {
                        tokens.push(Value::Part(
                            num_token.parse::<usize>().unwrap(),
                            (x, y, num_token.len()),
                        ))
                    }
                }
                '*' => {
                    // tokens.push(Value::Gear(x, y));
                    chars.next();
                }
                '.' => {
                    chars.next();
                }
                _ => {
                    // tokens.push(Value::Symbol(x, y));
                    chars.next();
                }
            }
        }
    }

    println!("{:?}", tokens);
    //
    // let mut k = tokens
    //     .iter()
    //     .filter_map(|(num, (x, y))| {
    //         let start_pos = (*x, *y);
    //         let end_pos = start_pos.0 + num.len();
    //
    //         let upper_bound_start = if start_pos.0 == 0 { 0 } else { start_pos.0 - 1 };
    //
    //         let top_row = if start_pos.1 == 0 {
    //             None
    //         } else {
    //             lines
    //                 .get(y - 1)
    //                 .map(|s| {
    //                     s.get(
    //                         upper_bound_start..if end_pos == s.len() {
    //                             end_pos
    //                         } else {
    //                             end_pos + 1
    //                         },
    //                     )
    //                 })
    //                 .flatten()
    //                 .filter(|s| s.chars().find(|&x| x != '.').is_some())
    //         };
    //
    //         let left = if start_pos.0 == 0 {
    //             None
    //         } else {
    //             lines
    //                 .get(start_pos.1)
    //                 .map(|s| s.get(start_pos.0 - 1..start_pos.0))
    //                 .flatten()
    //                 .filter(|s| s.chars().find(|&x| x != '.').is_some())
    //         };
    //
    //         let right = lines
    //             .get(start_pos.1)
    //             .map(|s| s.get(end_pos..end_pos + 1))
    //             .flatten()
    //             .filter(|s| s.chars().find(|&x| x != '.').is_some());
    //
    //         let bottom_row = lines
    //             .get(y + 1)
    //             .map(|s| {
    //                 s.get(
    //                     upper_bound_start..if end_pos == s.len() {
    //                         end_pos
    //                     } else {
    //                         end_pos + 1
    //                     },
    //                 )
    //             })
    //             .flatten()
    //             .filter(|s| s.chars().find(|&x| x != '.').is_some());
    //
    //         return [top_row, left, right, bottom_row]
    //             .iter()
    //             .find(|&x| x.is_some())
    //             .map(|_| num.clone());
    //     })
    //     .collect::<Vec<String>>();
    //
    // c.append(&mut k);
    //
    // return c.iter().map(|v| v.parse::<u32>().unwrap()).sum();
    0
}

fn part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    // loop chars
    // take while ascii digit
    //   -> check neighbours for other symbols than .
    //   -> if found make Some(ascii_digits) else None
    let mut tokens: Vec<(String, (usize, usize))> = vec![];

    let mut c: Vec<String> = vec![];

    for (y, s) in lines.iter().enumerate() {
        let mut chars = s.char_indices().peekable();
        while let Some(&(x, c)) = chars.peek() {
            match c {
                '0'..='9' => {
                    let num_token = iter::from_fn(|| chars.next_if(|(_, c)| c.is_ascii_digit()))
                        .map(|(_, c)| c)
                        .collect::<String>();
                    tokens.push((num_token, (x, y)))
                }
                _ => {
                    chars.next();
                }
            }
        }
    }

    let mut k = tokens
        .iter()
        .filter_map(|(num, (x, y))| {
            let start_pos = (*x, *y);
            let end_pos = start_pos.0 + num.len();

            let upper_bound_start = if start_pos.0 == 0 { 0 } else { start_pos.0 - 1 };

            let top_row = if start_pos.1 == 0 {
                None
            } else {
                lines
                    .get(y - 1)
                    .map(|s| {
                        s.get(
                            upper_bound_start..if end_pos == s.len() {
                                end_pos
                            } else {
                                end_pos + 1
                            },
                        )
                    })
                    .flatten()
                    .filter(|s| s.chars().find(|&x| x != '.').is_some())
            };

            let left = if start_pos.0 == 0 {
                None
            } else {
                lines
                    .get(start_pos.1)
                    .map(|s| s.get(start_pos.0 - 1..start_pos.0))
                    .flatten()
                    .filter(|s| s.chars().find(|&x| x != '.').is_some())
            };

            let right = lines
                .get(start_pos.1)
                .map(|s| s.get(end_pos..end_pos + 1))
                .flatten()
                .filter(|s| s.chars().find(|&x| x != '.').is_some());

            let bottom_row = lines
                .get(y + 1)
                .map(|s| {
                    s.get(
                        upper_bound_start..if end_pos == s.len() {
                            end_pos
                        } else {
                            end_pos + 1
                        },
                    )
                })
                .flatten()
                .filter(|s| s.chars().find(|&x| x != '.').is_some());

            return [top_row, left, right, bottom_row]
                .iter()
                .find(|&x| x.is_some())
                .map(|_| num.clone());
        })
        .collect::<Vec<String>>();

    c.append(&mut k);

    return c.iter().map(|v| v.parse::<u32>().unwrap()).sum();
}

fn main() {
    let input = include_str!("../../../input/day03");
    println!("answer part1: {}", part1(input));
    // println!("answer part2: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part1(test_input), 4361, "for input `{}`", test_input);
    }

    // #[test]
    // fn part_two_test() {
    //     let test_input = "";
    //     assert_eq!(part2(test_input), 8, "for input `{}`", test_input);
    // }
}
