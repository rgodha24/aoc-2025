advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid = Vec::new();
    for line in input.lines() {
        for range in line.split(",") {
            if range.is_empty() {
                continue;
            }
            let (lwoer, upper) = range.split_once('-').unwrap();
            let (lower, upper) = (lwoer.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap());
            for id in lower..=upper {
                let ids = id.to_string();

                if ids.is_empty() {
                    continue;
                }
                let id = ids.as_bytes();
                if id[..id.len() / 2] == id[id.len() / 2..] {
                    invalid.push(ids.parse::<u64>().unwrap());
                }
            }
        }
    }

    Some(invalid.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid = Vec::new();
    for line in input.lines() {
        for range in line.split(",") {
            if range.is_empty() {
                continue;
            }
            let (lwoer, upper) = range.split_once('-').unwrap();
            let (lower, upper) = (lwoer.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap());
            for id in lower..=upper {
                let ids = id.to_string();

                if ids.is_empty() {
                    continue;
                }
                let id = ids.as_bytes();
                for size in 1..id.len() {
                    if id.len() % size != 0 {
                        continue;
                    }
                    if (0..(id.len() / size))
                        .map(|n| n * size)
                        .map(|i| &id[i..i + size])
                        .collect::<Vec<_>>()
                        .windows(2)
                        .all(|w| w[0] == w[1])
                    {
                        invalid.push(ids.parse::<u64>().unwrap());
                        break;
                    }
                }
            }
        }
    }

    Some(invalid.iter().sum())
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
