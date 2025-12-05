use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct RangeSet {
    ranges: Vec<RangeInclusive<isize>>,
}

impl RangeSet {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn add(&mut self, new_range: RangeInclusive<isize>) {
        self.ranges.push(new_range);
        self.merge_ranges();
    }

    pub fn contains(&self, id: isize) -> bool {
        self.ranges.iter().any(|range| range.contains(&id))
    }

    fn merge_ranges(&mut self) {
        if self.ranges.is_empty() { return; }

        self.ranges.sort_by_key(|range| *range.start());

        let mut merged = Vec::new();
        let mut current_range = self.ranges[0].clone();

        for next_range in self.ranges.iter().skip(1) {
            if *next_range.start() <= *current_range.end() + 1 {
                let new_end = max(*current_range.end(), *next_range.end());
                current_range = *current_range.start()..=new_end;
            } else {
                merged.push(current_range);
                current_range = next_range.clone();
            }
        }

        merged.push(current_range);
        self.ranges = merged;
    }
}
#[aoc_generator(day5)]
fn parse(input: &str) -> (RangeSet, Vec<isize>) {
    let mut range_set: RangeSet = RangeSet::new();
    let mut ids: Vec<isize> = Vec::new();
    let (range_block, action_block) = input.split_once("\n\n").unwrap();

    range_block.lines().for_each(|line| {
        let (start, end) = line.split_once("-").unwrap();
        range_set.add(RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap()));
    });

    action_block.lines().for_each(|line| {
        ids.push(line.parse().unwrap());
    });


    (range_set, ids)
}

#[aoc(day5, part1)]
fn part1(input: &(RangeSet, Vec<isize>)) -> usize {
    let (range_set, ids) = input;

    ids.iter()
        .filter(|&&id| range_set.contains(id))
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &(RangeSet, Vec<isize>)) -> usize {
    let (range_set, ids) = input;

    range_set.ranges.iter()
        .map(|range| { ((range.end() - range.start()) + 1) as usize })
        .sum()
}
const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() { assert_eq!(part1(&parse(INPUT)), 3); }

    #[test]
    fn part2_example() { assert_eq!(part2(&parse(INPUT)), 14); }
}