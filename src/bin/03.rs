advent_of_code::solution!(3);
use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i64> {
    let mut sum = 0;
    for line in input.lines() {
        let nums: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let largest = *nums[..nums.len() - 1].iter().max().unwrap();
        let largest_index = nums.iter().position(|&x| x == largest).unwrap();
        let second_largest = nums.iter().skip(largest_index + 1).max().unwrap();
        sum += largest * 10 + second_largest;
    }

    Some(sum.into())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut sum = 0;
    for line in input.lines() {
        let nums: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut so_far = 0i64;
        let mut skip = 0;
        for i in 0..12 {
            let largest = *nums[..nums.len() - (11 - i)]
                .iter()
                .skip(skip)
                .max()
                .unwrap();
            let largest_index = nums.iter().skip(skip).position(|&x| x == largest).unwrap() + skip;
            so_far = so_far * 10 + largest as i64;
            skip = largest_index + 1;
        }
        sum += so_far;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
