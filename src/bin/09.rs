advent_of_code::solution!(9);
use std::{cmp, collections::HashSet};

use advent_of_code::helpers::*;
use itertools::Itertools;
use num::integer::gcd;

tiles!('.' => Empty, 'X' => Green,  '#' => Red);

pub fn part_one(input: &str) -> Option<i64> {
    let points = input
        .lines()
        .map(|line| SignedPoint::from_delimited(line, ",").unwrap())
        .collect_vec();
    Some(
        points
            .iter()
            .tuple_combinations()
            .map(|(a, b)| ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1))
            .max()
            .unwrap(),
    )
}

fn point_in_rect(recta: SignedPoint, rectb: SignedPoint, point: SignedPoint) -> bool {
    let SignedPoint { x: ax, y: ay } = recta;
    let SignedPoint { x: bx, y: by } = rectb;
    let SignedPoint { x: px, y: py } = point;
    ((ax <= px) && (px <= bx) || (ax >= px) && (px >= bx))
        && ((ay <= py) && (py <= by) || (ay >= py) && (py >= by))
}

pub fn part_two(input: &str) -> Option<i64> {
    let points = input
        .lines()
        .map(|line| SignedPoint::from_delimited(line, ",").unwrap())
        .collect_vec();

    let ans = points
        .iter()
        .copied()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            let minx = a.x.min(b.x);
            let miny = a.y.min(b.y);
            let maxx = a.x.max(b.x);
            let maxy = a.y.max(b.y);

            let vertices = points
                .iter()
                .cloned()
                .map(|p| SignedPoint::new(p.x.max(minx).min(maxx), p.y.max(miny).min(maxy)))
                .unique()
                .collect_vec();

            let shoelace = vertices
                .iter()
                .copied()
                .circular_tuple_windows()
                .map(|(a, b)| (a.x * b.y - (a.y * b.x)))
                .sum::<i64>()
                .abs()
                / 2;
            let lattice = vertices
                .iter()
                .copied()
                .chain(std::iter::once(vertices[0]))
                .tuple_windows()
                .map(|(a, b)| gcd(b.x - a.x, b.y - a.y))
                .sum::<i64>();

            let max_area = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
            let area = shoelace + lattice / 2 + 1;
            println!("area between {a} and {b} is {area}. max area is {max_area}");
            (area == max_area).then_some(area)
        })
        .max();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
