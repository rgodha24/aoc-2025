advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let mut dial = 50i64;
    let mut cnt = 0;
    for line in input.lines() {
        if line.chars().next().unwrap() == 'R' {
            let amt: i64 = line[1..].parse().unwrap();
            dial = (dial + amt).rem_euclid(100);
        } else {
            let amt: i64 = line[1..].parse().unwrap();
            dial = (dial - amt).rem_euclid(100);
        }
        if dial == 0 {
            cnt += 1;
        }
    }
    Some(cnt)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut dial = 50i64;
    let mut cnt = 0;
    for line in input.lines() {
        let prev = dial;
        let new;
        if line.chars().next().unwrap() == 'R' {
            let amt: i64 = line[1..].parse().unwrap();

            new = prev + amt;
        } else {
            let amt: i64 = line[1..].parse().unwrap();
            new = prev - amt;
        }
        if new.signum() != prev.signum() && prev != 0 {
            cnt += 1;
        }
        cnt += new.abs() / 100;
        dial = new.rem_euclid(100);
    }
    Some(cnt as usize)
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
        assert_eq!(result, Some(6));
    }
}
