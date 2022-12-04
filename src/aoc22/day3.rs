/// Our keyspace is the 52 characters [a-zA-Z] so I've decided to be cute and
/// use a u64 as my set. Each character's bit index is its priority
use anyhow::Result;
use std::io::BufRead;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{1} is an invalid {0}")]
    Invalid(&'static str, String),
}

const BIG_A: u8 = 'A' as u8;

fn priority(b: u8) -> u8 {
    if b >= ('a' as u8) {
        b - ('`' as u8)
    } else {
        27 + b - BIG_A
    }
}

fn hash_of(rucksack: &[u8]) -> u64 {
    let mut hash: u64 = 0;
    for item in rucksack {
        hash |= 1 << priority(*item);
    }
    hash
}

fn intersection(a: &[u8], b: &[u8]) -> usize {
    // There are 26*2 possible values for a valid input.
    let left = hash_of(a);

    for byte in b {
        let pri = priority(*byte);
        if left & (1 << pri) != 0 {
            return pri.into();
        }
    }
    unreachable!()
}

pub fn part1(rdr: impl BufRead) -> Result<usize> {
    Ok(rdr
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let bs = l.as_bytes();
            let priority = intersection(&bs[0..bs.len() / 2], &bs[bs.len() / 2..]);
            return priority;
        })
        .sum::<usize>())
}

pub fn part2(rdr: impl BufRead) -> Result<u64> {
    let lines: Vec<String> = rdr.lines().map(|l| l.unwrap()).collect();
    println!("");
    Ok(lines
        .chunks(3)
        .map(|arr| {
            assert_eq!(arr.len(), 3);
            let a = hash_of(arr[0].as_bytes());
            let b = hash_of(arr[1].as_bytes());
            let c = hash_of(arr[2].as_bytes());
            63 - (a & b & c).leading_zeros() as u64
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(Cursor::new(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ))?,
            157
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(Cursor::new(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ))?,
            70
        );
        Ok(())
    }
}
