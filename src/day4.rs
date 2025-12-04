use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let data = input.lines()
        .flat_map(|line| line.bytes())
        .collect();

    Grid { width, height, data }
}

pub fn get_elem(grid: &Grid, x: isize, y: isize) -> Option<u8> {
    if x < 0 || y < 0 || x as usize >= grid.width || y as usize >= grid.height {
        return None;
    }

    let index = (y as usize) * grid.width + (x as usize);
    Some(grid.data[index])
}

fn is_accessible_roll(grid: &Grid, x: isize, y: isize) -> bool {
    const DELTAS: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let mut count = 0;

    for (dx, dy) in DELTAS {
        if let Some(val) = get_elem(grid, x + dx, y + dy) {
            if val == b'@' {
                count += 1;
            }
        }
    }

    count < 4
}

fn remove_accessible_rolls(grid: &mut Grid) -> usize {
    let mut to_remove = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let current_x = x as isize;
            let current_y = y as isize;

            let current_val = get_elem(grid, current_x, current_y).unwrap();

            if current_val == b'@' {
                if is_accessible_roll(grid, current_x, current_y) {
                    to_remove.push((x, y));
                }
            }
        }
    }

    let count = to_remove.len();

    for (x, y) in to_remove {
        let index = y * grid.width + x;
        grid.data[index] = b'.';
    }

    count
}

#[aoc(day4, part1)]
fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    remove_accessible_rolls(&mut grid)
}

#[aoc(day4, part2)]
fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut count = 0;

    loop {
        let removed = remove_accessible_rolls(&mut grid);
        if removed == 0 {
            break;
        }
        count += removed;
    }

    count
}

const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }
    // real input solution

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 43);
    }
//     real input solution 10132
}