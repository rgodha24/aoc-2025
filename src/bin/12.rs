advent_of_code::solution!(12, 1);
use advent_of_code::helpers::*;
use itertools::Itertools;

tiles!('.' => Empty, '#' => Wall);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Size {
    w: usize,
    h: usize,
    counts: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.split("\n\n").collect_vec();
    let sizes = lines.pop().unwrap();
    let shapes = lines
        .into_iter()
        .map(|line| {
            let (_, shape) = line.split_once("\n").unwrap();
            Grid::<Tile>::from_chars(shape)
        })
        .collect_vec();
    let sizes = sizes
        .lines()
        .map(|line| {
            let (dims, counts) = line.split_once(": ").unwrap();
            let (w, h) = dims.split_once('x').unwrap();
            let w = w.parse::<usize>().unwrap();
            let h = h.parse::<usize>().unwrap();
            let counts = counts
                .split_whitespace()
                .map(|i| i.parse::<usize>().unwrap())
                .collect_vec();
            Size { w, h, counts }
        })
        .collect_vec();
    Some(
        sizes
            .iter()
            .filter(|s| {
                (s.w * s.h)
                    > s.counts
                        .iter()
                        .enumerate()
                        .map(|(i, count)| shapes[i].count(|t, _| *t == Tile::Wall) * count)
                        .sum::<usize>()
            })
            .count(),
    )
}
