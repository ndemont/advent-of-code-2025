use aoc_runner_derive::{aoc, aoc_generator};

fn parse1(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let (pos1, val1) = get_max_digit(&line[..line.len() - 1]).unwrap();
            let (_pos2, val2) = get_max_digit(&line[pos1 + 1..]).unwrap();

            let string_val = format!("{}{}", val1 as char, val2 as char);

            println!("{}", string_val);
            string_val.parse::<usize>().unwrap()
        })
        .sum()
}

#[aoc_generator(day3)]
fn parse(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let mut string_val: String = String::new();
            let mut sliced_line = line;
            for i in 0..12 {
                if string_val.len() >= 12 {
                    break;
                }
                if sliced_line.len() <= 12 - i {
                    let (new_pos, new_val) = get_max_digit(&sliced_line[..12 - i]).unwrap();
                    sliced_line = &sliced_line[new_pos + 1..];
                    println!("{}", string_val);
                    string_val = format!("{}{}", string_val, new_val);
                } else {
                    println!("{}", string_val);
                    string_val = format!("{}{}", string_val, &sliced_line[..12 - string_val.len()]);
                    break;
                }
            }
            println!("{}", string_val);
            string_val.parse::<usize>().unwrap()
        })
        .sum()
}

fn get_max_digit(slice: &str) -> Option<(usize, u8)> {
    slice.as_bytes()
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_index, &byte)| byte)
        .map(|(index, &byte)| (index, byte))
}

#[aoc(day3, part1)]
fn part1(input: &usize) -> usize {
    *input
}

#[aoc(day3, part2)]
fn part2(input: &usize) -> usize {
    *input
}

const INPUT: &str = "811111111111119
234234234234278
818181911112111";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(INPUT)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part1(&parse(INPUT)), 3121910778619);
    }
}