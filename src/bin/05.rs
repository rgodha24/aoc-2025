advent_of_code::solution!(5);
use std::{cmp::Ordering, collections::HashSet};

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, fresh) = input.split_once("\n\n").unwrap();
    let mut all: HashSet<_> = fresh
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    let mut fresh = HashSet::new();

    let ranges = ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect_vec();

    for &f in all.iter() {
        for &(a, b) in ranges.iter() {
            if a <= f && f <= b {
                fresh.insert(f);
            }
        }
    }

    Some(fresh.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ranges, fresh) = input.split_once("\n\n").unwrap();
    let all: HashSet<_> = fresh
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut ranges = ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect_vec();

    ranges.sort();

    let v = ranges.iter().fold(Vec::new(), |mut acc, element| {
        dbg!(element, &acc);
        let len = acc.len();
        let Some(last) = acc.last_mut() else {
            acc.push(*element);
            return acc;
        };

        // last=a,b, element=c,d
        // invariant: a <= c
        //
        // ab cd -- append cd
        // a c b d -- update last
        // a cd b -- noop

        if last.1 < element.0 {
            acc.push(*element);
        } else if last.0 <= element.0 && last.1 >= element.1 {
            dbg!(last, element);
            // completely covered, so noop
        } else if last.0 <= element.0 {
            last.1 = element.1;
        } else {
            dbg!(last, element);
        }
        return acc;

        // match (last.0.cmp(&element.0), last.1.cmp(&element.1)) {
        //     (Ordering::Less, Ordering::Less) => {
        //         acc.push(*element);
        //         dbg!((last, element));
        //         acc[len - 1] = (last.0, element.1);
        //     }
        //     (Ordering::Less, Ordering::Equal) => {}
        //     (Ordering::Less, Ordering::Greater) => {}
        //     (Ordering::Equal, Ordering::Equal) => {}
        //     (Ordering::Equal, Ordering::Greater) => {}
        //     _ => panic!("invalid sort"),
        // };
        //
        // acc
    });

    dbg!(&v);

    Some(v.into_iter().map(|x| (x.0..=x.1).count()).sum())
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
