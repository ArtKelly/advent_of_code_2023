use crate::{Solution, SolutionPair};
use grid::Grid;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, PartialEq)]

struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Part {
    location: Point,
    part_type: char,
}

fn create_grid(input: &str) -> Grid<char> {
    // Create a grid and fill it with our input
    let mut grid: Grid<char> = Grid::new(0, 0);
    for l in input.lines() {
        grid.push_row(l.chars().collect_vec());
    }
    grid
}

fn parse_part(location: (usize, usize), part_type: &char) -> Option<Part> {
    match part_type {
        // Filter out any dots or numbers, keeping all the parts
        '0'..='9' | '.' => None,
        _ => Some(Part {
            part_type: *part_type,
            location: Point {
                x: location.0,
                y: location.1,
            },
        }),
    }
}

fn find_adjacent_points(point: &Point) -> Vec<Point> {
    // For the part, calculate all it's adjacent coordinates
    let mut adjacent_points: Vec<Point> = vec![];

    // Loop around the point generating a vec
    // There are no parts on the edge of the schematic so we do not worry about over/underflowing
    for x in (point.x - 1)..=point.x + 1 {
        for y in (point.y - 1)..=(point.y + 1) {
            adjacent_points.push(Point { x, y });
        }
    }

    adjacent_points
}

fn discover_numbers(part: &Part, grid: &Grid<char>) -> Vec<u32> {
    // For the part, calculate all it's adjacent coordinates
    let adjacent_points = find_adjacent_points(&part.location);

    // Regex matcher for numbers
    let re = Regex::new(r"\d+").unwrap();
    let mut matches: Vec<u32> = vec![];

    for x in (part.location.x - 1)..=part.location.x + 1 {
        // build string from the grid
        let row = grid.iter_row(x).collect::<String>();

        // Iterate through the matches and attach them to the part
        for m in re.find_iter(&row) {
            let match_range = m.start()..m.end();

            for y in match_range {
                if adjacent_points.contains(&Point { x, y }) {
                    //Parse the match and push the result into the part
                    matches.push(m.as_str().parse::<u32>().unwrap());
                    // Move onto the next regex match if a gear is touching
                    break;
                }
            }
        }
    }

    matches
}

fn get_parts_list(grid: Grid<char>) -> Vec<(Part, Vec<u32>)> {
    grid.indexed_iter()
        .filter_map(|(location, part_type)| parse_part(location, part_type))
        .map(|part| {
            // Discover the matches for the part
            let matches = discover_numbers(&part, &grid);
            (part, matches)
        })
        .collect_vec()
}

fn part_a(parts: Vec<(Part, Vec<u32>)>) -> u32 {
    parts
        .iter()
        .map(|(_, matches)| matches.iter().sum::<u32>())
        .sum::<u32>()
}

fn part_b(parts: Vec<(Part, Vec<u32>)>) -> u32 {
    parts
        .iter()
        .filter_map(|(part, matches)| match part.part_type {
            // Find the gears
            '*' => {
                // A gear must have two numbers
                if matches.len() == 2 {
                    Some(matches.iter().product::<u32>())
                } else {
                    None
                }
            }
            _ => None,
        }) //
        .sum::<u32>()
}

pub fn solve() -> SolutionPair {
    // Build out the grid and match all the numbers to the parts
    let grid = create_grid(include_str!("../../input/day3.txt"));
    let parts: Vec<(Part, Vec<u32>)> = get_parts_list(grid);

    (
        Solution::from(part_a(parts.clone())),
        Solution::from(part_b(parts.clone())),
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
    fn test_day_3() {
        let grid = create_grid(TEST_INPUT);
        let parts: Vec<(Part, Vec<u32>)> = get_parts_list(grid);
        assert_eq!(part_a(parts.clone()), 4361);
        assert_eq!(part_b(parts.clone()), 467835);
    }
}
