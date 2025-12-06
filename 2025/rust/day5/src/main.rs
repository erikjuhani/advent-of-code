use std::ops::Range;

fn main() {
    println!("DAY5 p1: {}", p1(include_str!("../../../input/day5")));
    println!("DAY5 p2: {}", p2(include_str!("../../../input/day5")));
}

fn p1(input: &str) -> usize {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .lines()
        .map(|range_raw| {
            let (start, end) = range_raw.split_once('-').unwrap();
            start.parse::<usize>().unwrap()..end.parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut r = vec![];
    let mut cur_range = Range::default();
    for (i, range) in ranges.iter().enumerate() {
        if i == 0 {
            cur_range = range.clone();
            continue;
        }
        if range.start > cur_range.end {
            // We store the previous range and create new range that we expand
            r.push(cur_range);
            cur_range = range.clone();
            continue;
        }

        cur_range.end = range.end;
        if i == ranges.len() - 1 {
            r.push(cur_range.clone());
        }
    }

    let ids = ids.lines().flat_map(|id| id.parse::<usize>());

    ids.filter(|id| {
        ranges
            .iter()
            .any(|range| *id >= range.start && *id <= range.end)
    })
    .count()
}

fn p2(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .lines()
        .map(|range_raw| {
            let (start, end) = range_raw.split_once('-').unwrap();
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let (last, mut r) =
        ranges
            .into_iter()
            .fold((0..0, vec![]), |(current, mut r), (start, end)| {
                if start < current.end {
                    (current.start..current.end.max(end + 1), r)
                } else {
                    r.push(current.clone());
                    (start..end + 1, r)
                }
            });

    r.push(last);

    r.iter().cloned().flatten().count()
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(3, p1(input))
    }

    #[test]
    fn p2_test() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(14, p2(input))
    }
}
