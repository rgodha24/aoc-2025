advent_of_code::solution!(6);
use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().collect_vec();
    let ops = lines.pop().unwrap();
    let ops = ops.split_whitespace().collect_vec();

    let nums = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let nums = transpose(nums);

    Some(
        ops.into_iter()
            .zip(nums)
            .map(|(op, nums)| match op {
                "+" => nums.iter().copied().sum::<usize>() as u64,
                "*" => nums.iter().copied().product::<usize>() as u64,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().collect_vec();
    let mut ops = lines.pop().unwrap().chars().enumerate();
    let lines = lines
        .into_iter()
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let mut prev_op = None;
    let mut nums = Vec::new();
    let mut sum = 0u64;
    loop {
        let op = ops.next();
        if matches!(op, None | Some((_, '+' | '*'))) {
            match prev_op {
                Some('+') => {
                    sum += nums.iter().copied().sum::<u64>();
                }
                Some('*') => {
                    sum += nums.iter().copied().product::<u64>();
                }
                _ => {}
            }
            nums.clear();
            if let Some((_, last_char)) = op {
                prev_op = Some(last_char);
            } else {
                break;
            }
        }
        let i = op.unwrap().0;

        let mut num = 0;
        for line in lines.iter() {
            match line[i].to_digit(10) {
                Some(x) => num = num * 10 + x as u64,
                None => {}
            }
        }
        if num != 0 {
            nums.push(num);
        }
    }

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
