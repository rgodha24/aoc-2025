advent_of_code::solution!(7);
use advent_of_code::helpers::*;
use std::collections::{HashMap, HashSet};

tiles!('.' => Empty, '^' => Splitter, 'S' => Start);

pub fn part_one(input: &str) -> Option<i64> {
    let grid = Grid::<Tile>::from_chars(input);
    let mut splitters = HashSet::new();
    let start = grid.find(Tile::Start).next().unwrap();
    splitters.insert(start);
    let mut splits = 0;
    while grid.contains_point(splitters.iter().next().unwrap().clone()) {
        let mut new_splitters = HashSet::new();
        for point in splitters {
            let below = point + Direction::Down;
            if matches!(grid.get(below), Some(Tile::Splitter)) {
                new_splitters.insert(below + Direction::Right);
                new_splitters.insert(below + Direction::Left);
                splits += 1;
            } else {
                new_splitters.insert(below);
            }
        }

        splitters = new_splitters;
    }
    Some(splits)
}

fn dfs(grid: &Grid<Tile>, point: Point, cache: &mut HashMap<Point, usize>) -> usize {
    if let Some(count) = cache.get(&point) {
        return *count;
    }
    if !grid.contains_point(point) {
        return 1;
    }

    let below = point + Direction::Down;
    if matches!(grid.get(below), Some(Tile::Splitter)) {
        let right = below + Direction::Right;
        let left = below + Direction::Left;
        let amt = dfs(grid, right, cache) + dfs(grid, left, cache);
        cache.insert(point, amt);
        return amt;
    } else {
        let amt = dfs(grid, below, cache);
        cache.insert(point, amt);
        return amt;
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = Grid::<Tile>::from_chars(input);
    let start = grid.find(Tile::Start).next().unwrap();
    let mut cache = HashMap::new();
    return Some(dfs(&grid, start, &mut cache) as i64);
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
