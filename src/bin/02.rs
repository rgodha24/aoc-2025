advent_of_code::solution!(2);
use advent_of_code::helpers::*;
use itertools::Itertools;

const POWERSOF10: [u64; 11] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
];

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        for range in line.trim().split(",") {
            let (lower, upper) = range.trim().split_once('-').unwrap();
            let (lower, upper) = (lower.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap());

            for id in lower..=upper {
                let digs = digits(id);
                if digs % 2 == 1 {
                    // TODO: figure out why this doesnt work lol (obv with a while loop instead of a for loop)
                    // id = POWERSOF10[digs as usize + 1];
                    continue;
                }

                let factor = POWERSOF10[digs as usize / 2];
                if id / factor == id % factor {
                    sum += id;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        for range in line.trim().split(",") {
            let (lower, upper) = range.trim().split_once('-').unwrap();
            let (lower, upper) = (lower.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap());

            for id in lower..=upper {
                let digs = digits(id) as usize;
                for size in 1..digs {
                    if digs % size != 0 {
                        continue;
                    }

                    let factor = POWERSOF10[size];
                    if (0..(digs / size))
                        .map(|i| (id / POWERSOF10[size * i]) % factor)
                        .tuple_windows()
                        .all(|(a, b)| a == b)
                    {
                        sum += id;
                        break;
                    }
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
