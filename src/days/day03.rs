use crate::{Solution, SolutionPair};
use grid::Grid;
use itertools::Itertools;
use regex::Regex;

fn part_a(input: &str) -> u32 {
    // Create a grid and fill it with our input
    let mut grid: Grid<char> = Grid::new(0, 0);
    for l in input.lines() {
        grid.push_row(l.chars().collect_vec());
    }

    // Regex matcher for numbers
    let re = Regex::new(r"\d+").unwrap();

    grid.iter_rows()
        .enumerate()
        // For each row, find the numbers.
        .map(|(row, row_chars)| {
            re.find_iter(&row_chars.collect::<String>())
                // Of the numbers, create a vec of adjacent points
                .filter_map(|m| {
                    let x_range = m.start().saturating_sub(1)..m.end().saturating_add(1);
                    let y_range = row.saturating_sub(1)..=row.saturating_add(1);
                    let mut adjacent_points: Vec<(usize, usize)> = vec![];

                    for col in x_range {
                        for row in y_range.clone() {
                            adjacent_points.push((row, col));
                        }
                    }

                    let mut is_part = false;

                    // For each adjacent point, check if it is a part. If so, break the loop
                    for next in adjacent_points {
                        match grid.get(next.0, next.1) {
                            // Ignore non parts
                            Some('0'..='9' | '.') => (),
                            None => (),
                            _ => {
                                is_part = true;
                                break;
                            }
                        };
                    }

                    // Match on the resulting check, parse the number and remove non-parts from the iterator
                    match is_part {
                        true => Some(m.as_str().parse::<u32>().unwrap()),
                        false => None,
                    }
                })
                // Sum up all the parts for that row
                .sum::<u32>()
        })
        // Sum up parts for all rows
        .sum::<u32>()
}

fn part_b(input: &str) -> u32 {
    // Create a grid and fill it with our input
    let mut grid: Grid<char> = Grid::new(0, 0);
    for l in input.lines() {
        grid.push_row(l.chars().collect_vec());
    }

    let re: Regex = Regex::new(r"\d+").unwrap();
    // Collect the (row, col) values of each part

    grid.indexed_iter()
        .filter_map(|(i, x)| match x {
            // Filter out any dots or numbers, keeping all the parts
            '*' => Some(i),
            _ => None,
        })
        .filter_map(|gear| {
            // For the gear, calculate all it's adjacent coordinates
            let mut adjacent_points: Vec<(usize, usize)> = vec![];
            for row in (gear.0 - 1)..=(gear.0 + 1) {
                for col in (gear.1 - 1)..=(gear.1 + 1) {
                    adjacent_points.push((row, col));
                }
            }

            let mut matches: Vec<u32> = vec![];

            // For the rows around the gear, regex match on the numbers and if the range of the match contains an adjacent point to the gear, add it to a vec
            for r in (gear.0 - 1)..=(gear.0 + 1) {
                let row = grid.iter_row(r).collect::<String>();
                for m in re.find_iter(&row) {
                    let range = m.start()..m.end();
                    for c in range {
                        if adjacent_points.contains(&(r, c)) {
                            matches.push(m.as_str().parse::<u32>().unwrap());
                            // Move onto the next regex match if a gear is touching
                            break;
                        }
                    }
                }
            }

            // If the number of matches on a gear is 2, return the gear ratio as its product
            if matches.len() == 2 {
                Some(matches.iter().product::<u32>())
            } else {
                None
            }
            // matches
        })
        .sum::<u32>()
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    (
        Solution::from(part_a(include_str!("../../input/day3.txt"))),
        Solution::from(part_b(include_str!("../../input/day3.txt"))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, 467835);
    }
}
