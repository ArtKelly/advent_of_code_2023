use crate::{Solution, SolutionPair};

fn get_input() -> &'static str {
    include_str!("../../input/day4.txt")
}

fn parse_line(input: &str) -> u32 {
    // Can safely unwrap as the Scratchcard format will always have a : & |
    let (_, input) = input.split_once(":").unwrap();
    let (left, right) = input.split_once('|').unwrap();

    let winning_numbers: Vec<u32> = left
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    // Fold over the right hand values and calculate how many winning numbers there are
    right
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .fold(0_u32, |acc, num| {
            if winning_numbers.contains(&num) {
                return acc + 1;
            }
            acc
        })
}

fn part_a(input: &str) -> u32 {
    // For each line calculate the score by working out 2 ^ (no_wins - 1), return sum
    input.lines().map(parse_line).fold(0, |acc, count| {
        if count != 0 {
            return acc + 2_u32.pow(count - 1);
        }
        acc
    })
}

fn part_b(input: &str) -> u32 {
    let mut res = vec![1; input.lines().count()];

    for (index, line) in input.lines().enumerate() {
        // For each score, increment the result vec by the number of scorecards at this index
        for x in 1..=parse_line(line) {
            res[index + x as usize] += res[index];
        }
    }

    // Sum the resulting number of scorecards
    res.iter().sum::<u32>()
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_a(get_input())),
        Solution::from(part_b(get_input())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, 30);
    }
}
