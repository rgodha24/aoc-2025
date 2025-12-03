advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| solve_line(line, 2)).sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| solve_line(line, 12)).sum::<u64>())
}

fn solve_line(line: &str, chars: usize) -> u64 {
    let nums: Vec<_> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let mut so_far = 0;
    let mut skip = 0;
    for i in 0..chars {
        let (largest_index, largest) = nums[..nums.len() - (chars - i - 1)]
            .iter()
            .copied()
            .enumerate()
            .skip(skip)
            // max_by_key returns the last element if there are multiple. We want the first.
            .max_by_key(|&(i, x)| x << 32 | (nums.len() - i) as u64)
            .unwrap();
        so_far = so_far * 10 + largest;
        skip = largest_index + 1;
    }
    so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
