advent_of_code::solution!(6);
use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines().collect_vec();
    let last = lines.pop().unwrap();
    let mut nums = vec![Vec::<usize>::new(); lines[0].split_whitespace().count()];

    for line in lines {
        for (i, num) in line
            .split_whitespace()
            .enumerate()
            .map(|(i, x)| (i, x.parse::<usize>().unwrap()))
        {
            nums[i].push(num);
        }
    }

    let ops = last.split_whitespace().collect_vec();

    let mut sum = 0usize;

    for (i, op) in ops.into_iter().enumerate() {
        match op {
            "+" => sum += nums[i].iter().copied().sum::<usize>(),
            "*" => sum += nums[i].iter().copied().product::<usize>(),
            _ => {}
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines().collect_vec();
    let mut last = lines.pop().unwrap().chars().enumerate();
    let lines = lines
        .into_iter()
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let mut prev_op = None;
    let mut nums = Vec::new();
    let mut sum = 0usize;
    while let Some((i, last_char)) = last.next() {
        if matches!(last_char, '+' | '*') {
            dbg!(&nums);
            match prev_op {
                Some('+') => {
                    sum += nums.iter().copied().sum::<usize>();
                }
                Some('*') => {
                    sum += nums.iter().copied().product::<usize>();
                }
                _ => {}
            }
            prev_op = Some(last_char);
            nums = Vec::new();
        }

        let mut num = 0;
        for line in lines.iter() {
            match line[i].to_digit(10) {
                Some(x) => num = num * 10 + x as usize,
                None => {}
            }
        }
        if num != 0 {
            nums.push(num);
        }
    }
    dbg!(&nums);
    match prev_op {
        Some('+') => {
            sum += nums.iter().copied().sum::<usize>();
        }
        Some('*') => {
            sum += nums.iter().copied().product::<usize>();
        }
        _ => {}
    }
    return Some(sum);

    // for line in lines {
    //     for (i, num) in line
    //         .split_whitespace()
    //         .enumerate()
    //         .map(|(i, x)| (i, x.parse::<usize>().unwrap()))
    //     {
    //         nums[i].push(num);
    //     }
    // }
    //
    // let ops = last.split_whitespace().collect_vec();
    //
    // let mut sum = 0usize;
    //
    // for (i, op) in ops.into_iter().enumerate() {
    //     match op {
    //         "+" => sum += nums[i].iter().copied().sum::<usize>(),
    //         "*" => sum += nums[i].iter().copied().product::<usize>(),
    //         _ => {}
    //     }
    // }

    // Some(sum)
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
