advent_of_code::solution!(4);
use advent_of_code::helpers::*;
use itertools::Itertools;

tiles!('@' => Filled, '.' => Empty);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::<Tile>::from_chars(input);
    Some(removeable(&grid).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = Grid::<Tile>::from_chars(input);
    let mut sum = 0;
    while let remove = removeable(&grid)
        && !remove.is_empty()
    {
        for point in remove {
            grid[point] = Tile::Empty;
            sum += 1;
        }
    }
    Some(sum)
}

fn removeable(grid: &Grid<Tile>) -> Vec<Point> {
    grid.flat_iter()
        .filter(|&(tile, point)| {
            *tile == Tile::Filled
                && grid
                    .neighbors_of_diagonal(point)
                    .into_iter()
                    .map(|p| grid[p])
                    .filter(|t| *t == Tile::Filled)
                    .count()
                    < 4
        })
        .map(|(_, point)| point)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
