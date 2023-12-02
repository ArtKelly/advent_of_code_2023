use crate::{Solution, SolutionPair};
use nom::{
    bytes::complete::{tag, take, take_till},
    character::complete::{alpha1, space1, u32},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Default)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, data) = separated_list0(tag(", "), separated_pair(u32, space1, alpha1))(input)?;
    let mut game = Game::default();
    for (count, colour) in data {
        match colour {
            "red" => game.red += count,
            "green" => game.green += count,
            "blue" => game.blue += count,
            _ => (),
        }
    }
    Ok((input, game))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, _) = take_till(|c| c == ':')(input)?;
    let (input, _) = take(2_u8)(input)?;
    separated_list0(tag("; "), parse_game)(input)
}

fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, data) = parse_lines(line).unwrap();
            data.iter().all(|game| game.is_valid())
        })
        .enumerate()
        .map(|(i, e)| if e { i as u32 + 1 } else { 0 })
        .sum::<u32>()
}

fn part_b(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, data) = parse_lines(line).unwrap();
            let (max_red, max_green, max_blue) = data.iter().fold((0, 0, 0), |(r, g, b), game| {
                (r.max(game.red), g.max(game.green), b.max(game.blue))
            });

            max_red * max_green * max_blue
        })
        .sum::<u32>()
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_a(include_str!("../../input/day2.txt"))),
        Solution::from(part_b(include_str!("../../input/day2.txt"))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, 2286);
    }
}
