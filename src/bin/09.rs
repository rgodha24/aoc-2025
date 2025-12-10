advent_of_code::solution!(9);
use advent_of_code::helpers::*;
use either::Either;
use itertools::Itertools;

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

pub fn part_two(input: &str) -> Option<i64> {
    let points = input
        .lines()
        .map(|line| SignedPoint::from_delimited(line, ",").unwrap())
        .collect_vec();

    fn clip_edge(
        subject: Vec<SignedPoint>,
        is_inside: impl Fn(SignedPoint) -> bool,
        intersect: impl Fn(SignedPoint) -> SignedPoint,
    ) -> Vec<SignedPoint> {
        subject
            .iter()
            .copied()
            .circular_tuple_windows()
            .flat_map(|(s, e)| match (is_inside(s), is_inside(e)) {
                (false, false) => Either::Left([].into_iter()),
                (true, true) => Either::Right(Either::Left([e].into_iter())),
                (true, false) => Either::Right(Either::Left([intersect(s)].into_iter())),
                (false, true) => Either::Right(Either::Right([intersect(s), e].into_iter())),
            })
            .collect()
    }

    let ans = points
        .clone()
        .into_iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            let minx = a.x.min(b.x);
            let miny = a.y.min(b.y);
            let maxx = a.x.max(b.x);
            let maxy = a.y.max(b.y);

            let mut poly = points.clone();

            poly = clip_edge(poly, |p| p.x >= minx, |s| SignedPoint::new(minx, s.y));
            poly = clip_edge(poly, |p| p.x <= maxx, |s| SignedPoint::new(maxx, s.y));
            poly = clip_edge(poly, |p| p.y >= miny, |s| SignedPoint::new(s.x, miny));
            poly = clip_edge(poly, |p| p.y <= maxy, |s| SignedPoint::new(s.x, maxy));

            if poly.len() > 1 && poly.first() == poly.last() {
                poly.pop();
            }

            let shoelace = poly
                .iter()
                .circular_tuple_windows()
                .map(|(p1, p2)| p1.x * p2.y - p1.y * p2.x)
                .sum::<i64>()
                .abs()
                / 2;
            let perim = poly
                .iter()
                .circular_tuple_windows()
                .map(|(p1, p2)| (p1.x - p2.x).abs() + (p1.y - p2.y).abs())
                .sum::<i64>();

            let area = shoelace + perim / 2 + 1;
            let max_area = ((maxx - minx).abs() + 1) * ((maxy - miny).abs() + 1);

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
