use aoc_runner_derive::aoc;

fn get_max_batterie(slice: &str) -> Option<(usize, u8)> {
    slice.as_bytes()
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_index, &byte)| byte)
        .map(|(index, &byte)| (index, byte))
}

fn get_max_joltage(size: usize, bank: &str) -> usize {
    let mut remaining_bank = bank;
    // with capacity helpps to avoid reallocation, in our case we can reuse the same space
    let mut max_batteries = String::with_capacity(size);

    for i in 0..size {
        let len = remaining_bank.len();
        let remaining_batteries = size - i;
        let search_range = len - remaining_batteries;
        let searchable_bank = &remaining_bank[..=search_range];

        if let Some((pos, batterie)) = get_max_batterie(searchable_bank) {
            max_batteries.push(batterie as char);
            remaining_bank = &remaining_bank[pos + 1..];
        } else {
            break;
        }
    }

    max_batteries.parse().unwrap()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| get_max_joltage(2, line))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| get_max_joltage(12, line))
        .sum()
}

const INPUT: &str = "818181911112111
811111111111119
234234234234278
987654321111111";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 357);
    }
    // real input answer 17166

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 3121910778619);
    }
    // real input answer 169077317650774
}