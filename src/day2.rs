use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
pub struct Range {
    start: isize,
    end: isize,
}

const DIVIDERS: [usize; 5] = [2, 3, 5, 7, 9];

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Range> {
    let mut result = Vec::new();

    for l in input.lines() {
        let ranges_str = l.split(',');

        for range_raw in ranges_str {
            let range_cleaned = range_raw.trim();
            if range_cleaned.is_empty() {
                continue;
            }

            let mut parts = range_cleaned.split('-');

            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();

            let range = Range { start, end };
            result.push(range);
        }
    }

    result
}

#[aoc(day2, part1)]
fn part1(input: &[Range]) -> usize {
    let mut count: isize = 0;

    for range in input {
        for i in range.start..=range.end {
            let exp = (i.to_string().len() / 2) as u32;
            let tenth = 10_isize.pow(exp);

            if i / tenth == i  % tenth {
                count += i;
            }
        }
    }

    println!("{count}");
    count as usize
}

#[aoc(day2, part2)]
fn part2(input: &[Range]) -> isize {
    let mut count: isize = 0;

    for range in input {
        for i in range.start..=range.end {
            let line = i.to_string();
            let length = line.len();
            let mut invalid_id = false;

            for &div in &DIVIDERS {
                if length % div != 0 {
                    continue;
                }

                let chunk_size = length / div;
                let chunk_list: Vec<&[u8]> = line.as_bytes().chunks(chunk_size).collect();

                for chunk_pos in 0..div - 1 {
                    if chunk_list[chunk_pos] != chunk_list[chunk_pos + 1] {
                        break;
                    }
                    if chunk_pos == div -2 {
                        println!("Adding i={i} to count={count}");
                        invalid_id = true;
                        break;
                    }
                }

                if invalid_id {
                    count += i;
                    break;
                }
            }
        }

        println!();
    }

    println!("{count}");
    count
}

const INPUT1: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT1)), 4174379265);
    }
}