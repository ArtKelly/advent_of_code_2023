use crate::{Solution, SolutionPair};

fn part_a(_input: &str) -> u32 {
    0
}

fn part_b(_input: &str) -> u32 {
    0
}

pub fn solve() -> SolutionPair {
    (Solution::from(part_a("")), Solution::from(part_b("")))
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "";

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, 0);
    }
}
