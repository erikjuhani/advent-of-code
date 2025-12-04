fn main() {
    println!("DAY4 p1: {}", p1(include_str!("../../../input/day4")));
    println!("DAY4 p2: {}", p2(include_str!("../../../input/day4")));
}

const NEIGHBOURS: [(isize, isize); 8] = [
    // Top row
    (-1, -1), // x . .
    (-1, 0),  // . x .
    (-1, 1),  // . . x
    // Middle row
    (0, -1), // x . .
    (0, 1),  // . . x
    // Bottom row
    (1, -1), // x . .
    (1, 0),  // . x .
    (1, 1),  // . . x
];

fn calc_neighbours((row, col): (usize, usize), grid: &[Vec<u8>]) -> usize {
    NEIGHBOURS
        .iter()
        .filter_map(|(y, x)| {
            row.checked_add_signed(*y)
                .zip(col.checked_add_signed(*x))
                .filter(|(y, x)| *y < grid.len() && *x < grid[*y].len())
        })
        .map(|(y, x)| match grid[y][x] {
            b'@' => 1,
            _ => 0,
        })
        .sum()
}

fn to_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn p1(input: &str) -> usize {
    let grid = to_grid(input);
    let rows = grid.len();

    (0..rows).fold(0, |acc, y| {
        let cols = grid[y].len();
        acc + (0..cols)
            .filter_map(|x| match grid[y][x] {
                b'@' => {
                    let n = calc_neighbours((y, x), &grid);
                    if n < 4 { Some(()) } else { None }
                }
                _ => None,
            })
            .count()
    })
}

fn p2(input: &str) -> usize {
    let mut grid = to_grid(input);
    let rows = grid.len();

    let mut total = 0;
    loop {
        let z = (0..rows)
            .fold(vec![], |mut acc, y| {
                let cols = grid[y].len();
                let m = (0..cols)
                    .filter_map(|x| match grid[y][x] {
                        b'@' => {
                            let n = calc_neighbours((y, x), &grid);
                            if n < 4 { Some((y, x)) } else { None }
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                acc.push(m);
                acc
            })
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if !z.is_empty() {
            total += z.len();
            z.into_iter().for_each(|(y, x)| {
                if let Some(row) = grid.get_mut(y) {
                    row[x] = b'.';
                }
            });
        } else {
            break;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::{calc_neighbours, p1, p2, to_grid};

    #[test]
    fn calc_neighbours_test() {
        let grid = to_grid(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );

        let tests = [
            ((0usize, 0usize), 2),
            ((0usize, 2usize), 3),
            ((1usize, 1usize), 6),
            ((1usize, 4usize), 4),
        ];

        tests.into_iter().for_each(|((row, col), expected)| {
            assert_eq!(expected, calc_neighbours((row, col), &grid));
        });
    }

    #[test]
    fn p1_test() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(13, p1(input));
    }

    #[test]
    fn p2_test() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(43, p2(input));
    }
}
