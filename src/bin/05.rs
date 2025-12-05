advent_of_code::solution!(5);
use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, ingredients) = parse(input);

    Some(
        ingredients
            .into_iter()
            .filter(|i| match ranges.binary_search_by_key(i, |r| *r.start()) {
                Ok(_) => true,
                Err(0) => false,
                Err(idx) => ranges[idx - 1].contains(i),
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ranges, _) = parse(input);
    Some(ranges.into_iter().map(|x| x.count()).sum())
}

#[inline(always)]
fn parse(input: &str) -> (Vec<RangeInclusive<usize>>, impl IntoIterator<Item = usize>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect_vec();
    ranges.sort();

    let v = ranges.into_iter().fold(Vec::new(), |mut acc, (c, d)| {
        // last=a,b, element=c,d
        // invariant: a <= c
        //
        // ab cd -- append cd
        // a c b d -- update last
        // a cd b -- noop
        match acc.last_mut() {
            Some((_, b)) if *b < c => acc.push((c, d)),
            None => acc.push((c, d)),
            Some((a, b)) if *a <= c && *b < d => *b = d,
            _ => {}
        };
        acc
    });

    let ranges = v.into_iter().map(|(a, b)| a..=b).collect_vec();
    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<usize>().unwrap());
    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
