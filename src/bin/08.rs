advent_of_code::solution!(8);
use std::collections::HashMap;

use advent_of_code::helpers::*;
use itertools::Itertools;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect_vec(),
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        let p = self.find(self.parent[x]);
        self.parent[x] = p;
        p
    }
    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.parent[x] = y;
        }
    }
}

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
    let mut count = 0;
    dbg!(&connections[..3]);
    let mut uf = UnionFind::new(points.len());
    let point_to_index: HashMap<_, _> = points.iter().enumerate().map(|(i, p)| (p, i)).collect();

    for (distance, a, b) in connections {
        count += 1;
        if count > 1000 {
            break;
        }
        if uf.find(point_to_index[a]) == uf.find(point_to_index[b]) {
            // println!(
            //     "skipping {:?}->{:?} because they are already connected",
            //     a, b
            // );
            // dbg!(distance);

            continue;
        }

        uf.union(point_to_index[a], point_to_index[b]);
        println!("connected {:?}->{:?}", a, b);
        println!("their parent is now {:?}", uf.find(point_to_index[a]));
    }
    for i in 0..points.len() {
        uf.find(i);
    }
    // dbg!(&adj);
    // dbg!(&adj.values().map(|v| v.len()).collect_vec());
    dbg!(&count);
    dbg!(&uf.parent);
    dbg!(&uf.parent.iter().counts());
    Some(
        uf.parent
            .iter()
            .counts()
            .values()
            .into_iter()
            .sorted()
            .rev()
            .take(3)
            .product::<usize>(),
    )
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
    dbg!(&connections[..3]);
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
        if uf.parent.iter().counts().len() == 1 {
            return Some(a.0 * b.0);
        }
    }
    unreachable!();
    // dbg!(&adj);
    // dbg!(&adj.values().map(|v| v.len()).collect_vec());
    // dbg!(&count);
    // dbg!(&uf.parent);
    // dbg!(&uf.parent.iter().counts());
    // Some(
    //     uf.parent
    //         .iter()
    //         .counts()
    //         .values()
    //         .into_iter()
    //         .sorted()
    //         .rev()
    //         .take(3)
    //         .product::<usize>(),
    // )
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
