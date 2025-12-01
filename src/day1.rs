use aoc_runner_derive::{aoc, aoc_generator};

const DIAL_SIZE: isize = 100;

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Left { clicks: isize },
    Right { clicks: isize },
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| {
            let (rotation, clicks) = line.split_at(1);
            let clicks: isize = clicks.parse().expect("invalid number");

            match rotation {
                "L" => Rotation::Left { clicks },
                "R" => Rotation::Right { clicks },
                _ => panic!("invalid Rotation"),
            }
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Rotation]) -> usize {
    let mut position: isize = 50;
    let mut count: usize = 0;

    for rotation in input {
        let step = match rotation {
            Rotation::Left { clicks } => -clicks,
            Rotation::Right { clicks } => *clicks,
        };

        position = (position + step).rem_euclid(DIAL_SIZE);

        if position == 0 {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part2)]
pub fn part2(input: &[Rotation]) -> usize {
    let (total_hits, _) = input.iter().fold((0, 50), |(count, pos), rotation| {
        let (clicks, direction) = match rotation {
            Rotation::Right { clicks } => (*clicks, 1),
            Rotation::Left { clicks } => (*clicks, -1),
        };

        let dist_to_zero = if pos == 0 {
            DIAL_SIZE
        } else if direction == 1 {
            DIAL_SIZE - pos
        } else {
            pos
        };

        let hits = if clicks >= dist_to_zero {
            1 + (clicks - dist_to_zero) / DIAL_SIZE
        } else {
            0
        };

        let new_pos = (pos + clicks * direction).rem_euclid(DIAL_SIZE);

        (count + hits as usize, new_pos)
    });

    total_hits
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const INPUT2: &str = "L50
L100
R200
L100
R300";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT1)), 3);
        assert_eq!(part1(&input_generator(INPUT2)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT1)), 6);
        assert_eq!(part2(&input_generator(INPUT2)), 8);
    }
}
