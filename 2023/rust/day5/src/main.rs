fn part1(input: &str) -> usize {
    // destination range start, source range start, range length
    // 50 98 2
    // 50..51 98..99
    // Source numbers that are not mapped has the same destination number
    let x = input
        .split("\n\n")
        .filter(|&x| !x.is_empty())
        .map(|s| {
            s.split('\n')
                .map(|s| {
                    s.split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let seeds = x.first().and_then(|x| x.first()).unwrap();
    let maps = &x[1..]
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| (x[0]..(x[0] + x[2]), x[1]..(x[1] + x[2])))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|&s| {
            maps.iter().fold(s, |acc0, map| {
                map.iter()
                    .find(|(_, source)| source.contains(&acc0))
                    .map_or(acc0, |(dest, source)| dest.start + (acc0 - source.start))
            })
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    // destination range start, source range start, range length
    // 50 98 2
    // 50..51 98..99
    // Source numbers that are not mapped has the same destination number
    let x = input
        .split("\n\n")
        .filter(|&x| !x.is_empty())
        .map(|s| {
            s.split('\n')
                .map(|s| {
                    s.split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let seeds = x.first().and_then(|x| x.first()).unwrap();
    let h = seeds
        .windows(2)
        .step_by(2)
        .map(|w| (w[0]..(w[0] + w[1])))
        .flatten();

    let maps = &x[1..]
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| (x[0]..(x[0] + x[2]), x[1]..(x[1] + x[2])))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    h.map(|s| {
        maps.iter().fold(s, |acc0, map| {
            map.iter()
                .find(|(_, source)| source.contains(&acc0))
                .map_or(acc0, |(dest, source)| dest.start + (acc0 - source.start))
        })
    })
    .min()
    .unwrap()
}

fn main() {
    let input = include_str!("../../../input/day05");
    println!("answer part1: {}", part1(input));
    println!("answer part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_one_test() {
        assert_eq!(part1(TEST_INPUT), 35, "for input `{}`", TEST_INPUT);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part2(TEST_INPUT), 46, "for input `{}`", TEST_INPUT);
    }
}
