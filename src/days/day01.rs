use crate::{Solution, SolutionPair};

fn fix_lines(input: &str) -> String {
    let input = input.replace("one", "one1one");
    let input = input.replace("two", "two2two");
    let input = input.replace("three", "three3three");
    let input = input.replace("four", "four4four");
    let input = input.replace("five", "five5five");
    let input = input.replace("six", "six6six");
    let input = input.replace("seven", "seven7seven");
    let input = input.replace("eight", "eight8eight");
    input.replace("nine", "nine9nine")
}

fn part_a(input: &str) -> u32 {
    let res = input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .chars()
                .filter(|c| c.is_numeric())
                .map(|d| d.to_digit(10).unwrap())
                .collect();
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .sum::<u32>();
    res
}

fn part_b(input: &str) -> u32 {
    let res = input
        .lines()
        .map(fix_lines)
        .map(|line| {
            let numbers: Vec<u32> = line
                .chars()
                .filter(|c| c.is_numeric())
                .map(|d| d.to_digit(10).unwrap())
                .collect();
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .sum::<u32>();
    res
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_a(include_str!("../../input/day1.txt"))),
        Solution::from(part_b(include_str!("../../input/day1.txt"))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let result = part_a(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
        );
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );
        assert_eq!(result, 281);
    }
}
