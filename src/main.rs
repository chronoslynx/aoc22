use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufReader, Seek};

use aoc22;

fn main() -> Result<()> {
    for arg in env::args().skip(1) {
        let day = arg.parse::<u8>()?;
        println!("Day {}:", day);
        // This is incredibly tedious. There _has_ to be a better way...
        match arg.as_str() {
            "1" => {
                let mut rdr = BufReader::new(File::open("data/day2.txt")?);
                println!("\tPart 1: {}", aoc22::day1::part1(&mut rdr)?);
                _ = rdr.seek(std::io::SeekFrom::Start(0))?;
                println!("\tPart 2: {}", aoc22::day1::part2(&mut rdr)?);
            }
            "2" => {
                let mut rdr = BufReader::new(File::open("data/day2.txt")?);
                println!("\tPart 1: {}", aoc22::day2::part1(&mut rdr)?);
                _ = rdr.seek(std::io::SeekFrom::Start(0))?;
                println!("\tPart 2: {}", aoc22::day2::part2(rdr)?);
            }
            "3" => {
                let mut rdr = BufReader::new(File::open("data/day3.txt")?);
                println!("\tPart 1: {}", aoc22::day3::part1(&mut rdr)?);
                _ = rdr.seek(std::io::SeekFrom::Start(0))?;
                println!("\tPart 2: {}", aoc22::day3::part2(rdr)?);
            }
            _ => {
                panic!("unsupported day");
            }
        }
    }
    Ok(())
}
