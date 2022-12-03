use anyhow::Result;
use std::io::BufRead;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{1} is an invalid {0}")]
    Invalid(&'static str, String),
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(theirs: Self, ours: Self) -> usize {
        match (theirs, ours) {
            (Move::Rock, Move::Rock) => 1 + 3,
            (Move::Rock, Move::Paper) => 2 + 6,
            (Move::Rock, Move::Scissors) => 3 + 0,
            (Move::Paper, Move::Rock) => 1 + 0,
            (Move::Paper, Move::Paper) => 2 + 3,
            (Move::Paper, Move::Scissors) => 3 + 6,
            (Move::Scissors, Move::Rock) => 1 + 6,
            (Move::Scissors, Move::Paper) => 2 + 0,
            (Move::Scissors, Move::Scissors) => 3 + 3,
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(ParseError::Invalid("move", s.into())),
        }
    }
}

fn score(game: &str) -> Result<usize> {
    let mut split = game.split(' ');
    let their_move = Move::try_from(split.next().unwrap())?;
    let our_move = Move::try_from(split.next().unwrap())?;
    Ok(Move::score(their_move, our_move))
}

pub fn part1(rdr: impl BufRead) -> Result<usize> {
    Ok(rdr
        .lines()
        .map(|l| l.unwrap())
        .map(|l| score(&l).unwrap())
        .sum::<usize>())
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl TryFrom<&str> for Outcome {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(ParseError::Invalid("outcome", value.into())),
        }
    }
}

fn solve(game: &str) -> Result<usize> {
    let mut split = game.split(' ');
    let their_move = Move::try_from(split.next().unwrap())?;
    let outcome = Outcome::try_from(split.next().unwrap())?;
    Ok(match (their_move, outcome) {
        // X lose, Y draw, Z win
        (Move::Rock, Outcome::Lose) => Move::score(Move::Rock, Move::Scissors),
        (Move::Rock, Outcome::Draw) => Move::score(Move::Rock, Move::Rock),
        (Move::Rock, Outcome::Win) => Move::score(their_move, Move::Paper),
        (Move::Paper, Outcome::Lose) => Move::score(their_move, Move::Rock),
        (Move::Paper, Outcome::Draw) => Move::score(their_move, Move::Paper),
        (Move::Paper, Outcome::Win) => Move::score(their_move, Move::Scissors),
        (Move::Scissors, Outcome::Lose) => Move::score(their_move, Move::Paper),
        (Move::Scissors, Outcome::Draw) => Move::score(their_move, Move::Scissors),
        (Move::Scissors, Outcome::Win) => Move::score(their_move, Move::Rock),
    })
}

pub fn part2(rdr: impl BufRead) -> Result<usize> {
    Ok(rdr
        .lines()
        .map(|l| l.unwrap())
        .map(|l| solve(&l).unwrap())
        .sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(Cursor::new("A Y\nB X\nC Z".to_string()))?, 15);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(Cursor::new("A Y\nB X\nC Z".to_string()))?, 12);
        Ok(())
    }
}
