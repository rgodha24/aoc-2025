advent_of_code::solution!(11);
use std::collections::{HashMap, HashSet};

use advent_of_code::helpers::*;
use itertools::Itertools;

fn dfs<'a>(
    at: &'a str,
    goal: &'a str,
    visited: &mut HashSet<&'a str>,
    adj_list: &HashMap<&'a str, Vec<&'a str>>,
) -> usize {
    if visited.contains(at) {
        return 0;
    }
    if at == goal {
        return 1;
    }
    visited.insert(at);

    let Some(v) = adj_list.get(at) else {
        return 0;
    };

    let count = v.iter().map(|to| dfs(to, goal, visited, adj_list)).sum();
    visited.remove(at);
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut adj_list: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        adj_list.insert(left, Vec::from_iter(right.split_whitespace()));
    }

    let mut visited = HashSet::new();
    Some(dfs("you", "out", &mut visited, &adj_list))
}

fn dfs2<'a>(
    at: &'a str,
    visited: &mut HashSet<&'a str>,
    adj_list: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<(&'a str, Vec<&'a str>), usize>,
    visited_dac: bool,
    visited_fft: bool,
) -> usize {
    if visited.contains(at) {
        return 0;
    }
    if let Some(result) = cache.get(&(at, Vec::from_iter(visited.iter().cloned()))) {
        return *result;
    }
    if at == "out" {
        return usize::from(visited_dac && visited_fft);
    }

    visited.insert(at);

    let count = adj_list
        .get(at)
        .unwrap()
        .iter()
        .map(|to| {
            dfs2(
                to,
                visited,
                adj_list,
                cache,
                visited_dac || at == "dac",
                visited_fft || at == "fft",
            )
        })
        .sum();
    cache.insert((at, Vec::from_iter(visited.iter().cloned())), count);
    visited.remove(at);
    count
}

fn dfs3<'a>(
    at: &'a str,
    goal: &'a str,
    visited: &mut HashSet<&'a str>,
    cache: &mut HashMap<&'a str, usize>,
    adj_list: &HashMap<&'a str, Vec<&'a str>>,
) -> usize {
    if visited.contains(at) {
        return 0;
    }
    if at == goal {
        return 1;
    }
    if let Some(result) = cache.get(at) {
        return *result;
    }
    visited.insert(at);

    let Some(v) = adj_list.get(at) else {
        return 0;
    };

    let count = v.iter().map(|to| dfs(to, goal, visited, adj_list)).sum();
    visited.remove(at);
    cache.insert(at, count);
    count
}

fn has_cycles<'a>(
    at: &'a str,
    visited: &mut HashSet<&'a str>,
    finished: &mut HashSet<&'a str>,
    adj_list: &HashMap<&'a str, Vec<&'a str>>,
) -> bool {
    if finished.contains(at) {
        return false;
    }
    if visited.contains(at) {
        return true;
    }
    visited.insert(at);
    let result = adj_list
        .get(at)
        .unwrap()
        .iter()
        .any(|to| has_cycles(to, visited, finished, adj_list));
    finished.insert(at);
    result
}

pub fn top_sort<'a>(
    at: &'a str,
    adj_list: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    post_order: &mut Vec<&'a str>,
) {
    if visited.contains(at) {
        return;
    }
    visited.insert(at);
    for to in adj_list.get(at).unwrap_or(&Vec::new()) {
        top_sort(to, adj_list, visited, post_order);
    }
    post_order.push(at);
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut adj_list: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        adj_list.insert(left, Vec::from_iter(right.split_whitespace()));
    }

    // dbg!(&adj_list.len());
    // let mut can_reach_dac = HashSet::new();
    // let mut can_reach_fft = HashSet::new();
    // for left in adj_list.keys() {
    //     let mut visited = HashSet::new();
    //     if dfs(left, "dac", &mut visited, &adj_list) > 0 {
    //         can_reach_dac.insert(*left);
    //     }
    //     let mut visited = HashSet::new();
    //     if dfs(left, "fft", &mut visited, &adj_list) > 0 {
    //         can_reach_fft.insert(*left);
    //     }
    // }

    let mut post_order = Vec::new();
    top_sort("svr", &adj_list, &mut HashSet::new(), &mut post_order);
    post_order.reverse();
    // dbg!(&post_order);
    let mut visited = vec![0; post_order.len()];
    visited[0] = 1;
    for (i, at) in post_order.iter().enumerate() {
        for to in adj_list.get(at).unwrap_or(&Vec::new()) {
            visited[post_order.iter().position(|a| a == to).unwrap()] += visited[i];
        }
    }
    let svr_to_fft = visited[post_order.iter().position(|a| *a == "fft").unwrap()];

    let mut visited = vec![0; post_order.len()];
    visited[post_order.iter().position(|a| *a == "fft").unwrap()] = 1;
    for (i, at) in post_order.iter().enumerate() {
        for to in adj_list.get(at).unwrap_or(&Vec::new()) {
            visited[post_order.iter().position(|a| a == to).unwrap()] += visited[i];
        }
    }
    let fft_to_dac = visited[post_order.iter().position(|a| *a == "dac").unwrap()];

    let mut visited = vec![0; post_order.len()];
    visited[post_order.iter().position(|a| *a == "dac").unwrap()] = 1;
    for (i, at) in post_order.iter().enumerate() {
        for to in adj_list.get(at).unwrap_or(&Vec::new()) {
            visited[post_order.iter().position(|a| a == to).unwrap()] += visited[i];
        }
    }
    let dac_to_out = visited[post_order.iter().position(|a| *a == "out").unwrap()];

    return Some(svr_to_fft * fft_to_dac * dac_to_out);
    todo!();

    let mut visited = HashSet::new();
    let mut cache = HashMap::new();
    let start = dfs3("svr", "fft", &mut visited, &mut cache, &adj_list);
    // dbg!(start);
    let mut visited = HashSet::new();
    let mut cache = HashMap::new();
    let dac = dfs3("fft", "dac", &mut visited, &mut cache, &adj_list);
    // dbg!(dac);
    let mut visited = HashSet::new();
    let mut cache = HashMap::new();
    let out = dfs3("dac", "out", &mut visited, &mut cache, &adj_list);
    // dbg!(start, dac, out);
    // todo!();
    return Some(start * dac * out);

    // dbg!(dfs("svr", &mut visited, &adj_list));
    // let mut visited = HashSet::new();
    // let mut cache = HashMap::new();
    // Some(dfs2(
    //     "svr",
    //     &mut visited,
    //     &adj_list,
    //     &can_reach_fft,
    //     &can_reach_dac,
    //     &mut cache,
    //     false,
    //     false,
    // ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
