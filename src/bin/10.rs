advent_of_code::solution!(10);
use std::num::NonZeroU64;

use advent_of_code::helpers::*;
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

tiles!('.' => Off, '#' => On);

fn turn_on(indices: impl Iterator<Item = u64>) -> u64 {
    let mut acc = 0;
    for index in indices {
        acc = acc | (1 << index);
    }
    acc
}

fn dfsp1(
    state: u64,
    buttons: &[u64],
    goal: u64,
    depth: u64,
    max_depth: u64,
    cache: &mut FxHashMap<u64, NonZeroU64>,
    unreachable: &mut FxHashSet<(u64, u64)>,
) -> Option<NonZeroU64> {
    if unreachable.contains(&(state, depth)) {
        return None;
    }
    if let Some(result) = cache.get(&state) {
        return NonZeroU64::new(result.get() + depth);
    }
    if depth > max_depth {
        return None;
    }
    if state == goal {
        // println!("reached goal at depth={depth}. state={state:b}. goal={goal}");
        return NonZeroU64::new(depth);
    }

    let min = buttons
        .iter()
        .filter_map(|b| {
            let updated = state ^ b;
            // println!(
            //     "updated={updated:b} by applying button={b:b} to state={state:b} at depth={depth}"
            // );
            dfsp1(
                updated,
                buttons,
                goal,
                depth + 1,
                max_depth,
                cache,
                unreachable,
            )
        })
        .min();
    if let Some(min) = min {
        cache.insert(state, NonZeroU64::new(min.get() - depth).unwrap());
    } else {
        unreachable.insert((state, depth));
    }
    min
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    'outer: for line in input.lines() {
        let (initial, rest) = line.split_once(" ").unwrap();
        let (button, jiggle) = rest.split_once("{").unwrap();
        let state = initial[1..initial.len() - 1]
            .chars()
            .map(|c| Tile::from(c))
            .collect_vec();
        let goal = turn_on(
            state
                .into_iter()
                .enumerate()
                .filter_map(|(i, t)| (t == Tile::On).then_some(i as u64)),
        );
        let buttons = button
            .split("(")
            .map(|b| b.trim())
            .filter(|b| !b.is_empty())
            .map(|b| {
                turn_on(
                    b[..b.len() - 1]
                        .split(",")
                        .map(|b| b.trim().parse::<u64>().unwrap()),
                )
            })
            .collect_vec();

        let mut cache: FxHashMap<u64, _> = FxHashMap::default();
        let mut unreachable: FxHashSet<(u64, u64)> = FxHashSet::default();

        let result = dfsp1(0, &buttons, goal, 0, 1000, &mut cache, &mut unreachable);
        if let Some(result) = result {
            sum += result.get();
            continue 'outer;
        }
    }
    Some(sum)
}

fn dfs2(
    state: &mut [i64],
    buttons: &[Vec<usize>],
    mut max_depth: usize,
    depth: usize,
    cooked_states: &mut FxHashSet<Vec<i64>>,
) -> Option<usize> {
    if state.iter().any(|&s| s < 0) {
        return None;
    }
    if depth > max_depth {
        return Some(usize::MAX);
    }
    if state.iter().all(|&s| s == 0) {
        return Some(depth);
    }

    buttons
        .iter()
        .filter_map(|button| {
            for i in button {
                state[*i] -= 1;
            }
            let result = dfs2(state, buttons, max_depth, depth + 1, cooked_states);
            for i in button {
                state[*i] += 1;
            }
            if let Some(result) = result {
                if result < max_depth {
                    max_depth = result;
                    println!("setting max_depth to {max_depth}");
                }
                max_depth = max_depth.min(result)
            } else {
                cooked_states.insert(state.to_owned());
            }
            result
        })
        .min()
}
pub fn part_two(input: &str) -> Option<usize> {
    return None;
    Some(
        input
            .lines()
            .collect_vec()
            .par_iter()
            .map(|line| {
                let (initial, rest) = line.split_once(" ").unwrap();
                let (button, jiggle) = rest.split_once("{").unwrap();
                let state = initial[1..initial.len() - 1]
                    .chars()
                    .map(|c| Tile::from(c))
                    .collect_vec();
                let buttons = button
                    .split("(")
                    .map(|b| b.trim())
                    .filter(|b| !b.is_empty())
                    .map(|b| {
                        b[..b.len() - 1]
                            .split(",")
                            .map(|b| b.trim().parse::<usize>().unwrap())
                            .collect_vec()
                    })
                    .collect_vec();
                let mut jiggle = jiggle[..jiggle.len() - 1]
                    .split(",")
                    .map(|b| b.trim().parse::<i64>().unwrap())
                    .collect_vec();
                let mut cooked_states = Default::default();
                dbg!(&jiggle);
                dbg!(dfs2(&mut jiggle, &buttons, 500, 0, &mut cooked_states).unwrap())
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
