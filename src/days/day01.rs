use crate::{Solution, SolutionPair};

fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // Initialise a result variable with value zero
            let mut result = 0;

            // Work forwards through the string and add the value of the first number to our result
            for c in line.chars() {
                if c.is_numeric() {
                    result += c.to_digit(10).unwrap();
                    break;
                }
            }

            // Work backwards through the string. On discovering the final digit, multiply the current result by 10 to shift the value left. Finally, add the value of the last number to our result
            for c in line.chars().rev() {
                if c.is_numeric() {
                    result *= 10;
                    result += c.to_digit(10).unwrap();
                    break;
                }
            }

            // return the result
            result
        })
        .sum::<u32>()
}

fn part_b(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            //Create a blank string
            let mut current_slice = String::from("");
            let mut result = 0;

            for c in line.chars() {
                // If its a digit, break early.
                if c.is_numeric() {
                    result += c.to_digit(10).unwrap();
                    break;
                }

                //Push the current character into the working slice
                current_slice.push(c);

                //Match the current slice to find the first number written as a word
                let val: Option<u32> = match current_slice {
                    ref x if x.contains("one") => Some(1),
                    ref x if x.contains("two") => Some(2),
                    ref x if x.contains("three") => Some(3),
                    ref x if x.contains("four") => Some(4),
                    ref x if x.contains("five") => Some(5),
                    ref x if x.contains("six") => Some(6),
                    ref x if x.contains("seven") => Some(7),
                    ref x if x.contains("eight") => Some(8),
                    ref x if x.contains("nine") => Some(9),
                    _ => None,
                };

                // If there is one, add it to the result and return
                if let Some(val) = val {
                    result += val;
                    break;
                }
            }

            // Reset the slice
            let mut current_slice = String::from("");

            // Loop through in reverse
            for c in line.chars().rev() {
                // If its a digit, break early.
                if c.is_numeric() {
                    result *= 10;
                    result += c.to_digit(10).unwrap();
                    break;
                }

                //Push the current character into the working slice
                current_slice.push(c);

                // Match the string in reverse
                let val: Option<u32> = match current_slice {
                    ref x if x.contains("eno") => Some(1),
                    ref x if x.contains("owt") => Some(2),
                    ref x if x.contains("eerht") => Some(3),
                    ref x if x.contains("ruof") => Some(4),
                    ref x if x.contains("evif") => Some(5),
                    ref x if x.contains("xis") => Some(6),
                    ref x if x.contains("neves") => Some(7),
                    ref x if x.contains("thgie") => Some(8),
                    ref x if x.contains("enin") => Some(9),
                    _ => None,
                };

                // Push to the result
                if let Some(val) = val {
                    result *= 10;
                    result += val;
                    break;
                }
            }

            result
        })
        .sum::<u32>()
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
