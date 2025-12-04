fn main() {
    println!("DAY4 p1: {}", p1(include_str!("../../../input/day4")));
    // println!("DAY4 p2: {}", p2(include_str!("../../../input/day3")));
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

fn calc_neighbours((row, col): (usize, usize), grid: &[&[u8]]) -> usize {
    NEIGHBOURS
        .iter()
        .filter_map(|(y, x)| {
            row.checked_add_signed(*y)
                .zip(col.checked_add_signed(*x))
                .filter(|(y, x)| *y < grid.len() && *x < grid[*y].len())
        })
        .map(|(y, x)| {
            // eprintln!("{row}, {col}: {y},{x}: {}", char::from(grid[y][x]));
            match grid[y][x] {
                b'@' => 1,
                _ => 0,
            }
        })
        .sum()
}

fn to_grid(input: &str) -> Vec<&[u8]> {
    input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>()
}

fn p1(input: &str) -> usize {
    // cursor: (row, col)
    //
    // -> move until we meet an edge (max col = line.len)
    // -> increment row
    //
    // roll of paper can only be accesd if fewer max 3 adjacent roll of papers
    // must check eight adjacent positions so
    // (y-1,x-1), (y-1, x) (y-1,x+1)
    // (y  ,x-1),          (y  ,x+1)
    // (y+1,x-1), (y+1, x) (y+1,x+1)
    //
    // -> check current cursor (y, x) position
    //
    // What if we are at edge?
    //
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

    // let mut amount = vec![];
    //
    // for (y, row) in input.lines().enumerate() {
    //     for (x, c) in row.char_indices() {
    //         if c == '@' {
    //             let n = calc_neighbours((y, x), &grid);
    //
    //             if n < 4 {
    //                 amount.push(Some(()));
    //             }
    //         }
    //     }
    // }
    //
    // amount.len()
}

#[cfg(test)]
mod tests {
    use crate::{calc_neighbours, p1, to_grid};

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
}
