advent_of_code::solution!(8);
use advent_of_code::helpers::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut connections = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (x1, y1, z1) = a;
            let (x2, y2, z2) = b;
            let distance = (((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as f64).sqrt();
            (distance, a, b)
        })
        .collect_vec();
    connections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut uf = UnionFind::new(points.len());
    let point_to_index: HashMap<_, _> = points.iter().enumerate().map(|(i, p)| (p, i)).collect();

    for (_, a, b) in connections.into_iter().take(1000) {
        uf.union(point_to_index[a], point_to_index[b]);
    }
    // make sure parents are all updated
    for i in 0..points.len() {
        uf.find(i);
    }
    let mut counts = uf.counts();
    counts.sort_unstable();
    Some(counts.into_iter().rev().take(3).product::<usize>())
}

pub fn part_two(input: &str) -> Option<i64> {
    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut connections = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (x1, y1, z1) = a;
            let (x2, y2, z2) = b;
            let distance = (((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as f64).sqrt();
            (distance, a, b)
        })
        .collect_vec();
    connections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut uf = UnionFind::new(points.len());
    let point_to_index: HashMap<_, _> = points.iter().enumerate().map(|(i, p)| (p, i)).collect();

    for (distance, a, b) in connections {
        if uf.find(point_to_index[a]) == uf.find(point_to_index[b]) {
            continue;
        }

        uf.union(point_to_index[a], point_to_index[b]);
        for i in 0..points.len() {
            uf.find(i);
        }

        let counts = uf.counts();
        let first = counts[0];
        if counts.into_iter().all(|x| x == first) {
            return Some(a.0 * b.0);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
