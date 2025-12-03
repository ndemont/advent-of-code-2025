use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> usize {
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

// #[aoc(day3, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 357);
    }
    //
    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}