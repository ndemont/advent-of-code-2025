use std::path::Component::ParentDir;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<char>, Vec<isize>) {
    let mut directions = Vec::new();
    let mut turns = Vec::new();

    for l in input.lines() {
        let mut chars = l.chars();

        directions.push(chars.next().unwrap());
        turns.push(chars.as_str().parse().unwrap());
    }

    (directions, turns)
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<char>, Vec<isize>)) -> usize {
    let mut new_position: usize = 50;
    let mut count = 0;

    let directions = &input.0;
    let turns = &input.1;

    for i in 0..directions.len() {
        let dir = directions[i];
        let step : isize = turns[i];
        let mut tmp_pos: isize;

        if dir == 'L' {
            tmp_pos = new_position as isize - step;
            while tmp_pos < 0 {
                println!("{}{}", dir, step);
                tmp_pos = 100 + tmp_pos;
            }
            new_position = tmp_pos as usize;
        } else {
            tmp_pos = new_position as isize + step;
            while tmp_pos > 99 {
                println!("{}{}", dir, step);
                tmp_pos = 100usize.abs_diff(tmp_pos as usize) as isize;
            }
            new_position = tmp_pos as usize
        }

        if new_position == 0 {
            count += 1
        }
    }

    println!("{}", count);
    count
}


#[aoc(day1, part2)]
pub fn part2(input: &(Vec<char>, Vec<isize>)) -> usize {
    let mut new_position: usize = 50;
    let mut count = 0;

    let directions = &input.0;
    let turns = &input.1;

    for i in 0..directions.len() {
        let dir = directions[i];
        let step : isize = turns[i];
        count += step / 100;
        if step / 100 != 0 {
            println!("+ {}: {}{}\n", step / 100, dir, step)
        }
        let mut tmp_pos: isize = step % 100;

        if dir == 'L' {
            tmp_pos = new_position as isize - tmp_pos;
            if tmp_pos < 0 {
                if new_position != 0 {
                    println!("+ 1: {}{}\n", dir, step);
                    count += 1;
                }
                tmp_pos = 100 + tmp_pos;
            } else if tmp_pos == 0 && step % 100 != 0 {
                println!("new_pos: {}, tmp_pos: {}", new_position, tmp_pos);
                println!("+ 1: {}{}\n", dir, step);
                count += 1;
            }
            new_position = tmp_pos as usize;
        } else {
            tmp_pos = new_position as isize + tmp_pos;
            if tmp_pos > 99 {
                if new_position != 0 {
                    println!("+ 1: {}{}\n", dir, step);
                    count += 1;
                }
                println!("+ 1: {}{}\n", dir, step);
                tmp_pos = 100usize.abs_diff(tmp_pos as usize) as isize;
            }
            new_position = tmp_pos as usize
        }
    }

    println!("{}", count);
    count as usize
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
        assert_eq!(3, part1(&input_generator(INPUT1)));
        assert_eq!(5, part1(&input_generator(INPUT2)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(&input_generator(INPUT1)));
        assert_eq!(8, part2(&input_generator(INPUT2)));
    }
}
