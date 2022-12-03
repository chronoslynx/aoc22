use anyhow::{Context, Result};
use std::collections::BinaryHeap;
use std::io::BufRead;

fn day1(rdr: impl BufRead) -> Result<BinaryHeap<usize>> {
    let mut heap = BinaryHeap::new();
    let mut acc: usize = 0;

    for line in rdr.lines() {
        let line = line?;
        if line.is_empty() {
            heap.push(acc);
            acc = 0;
            continue;
        }
        acc += line
            .parse::<usize>()
            .with_context(|| format!("Failed to parse {}", line))?;
    }
    heap.push(acc);
    Ok(heap)
}

pub fn part1(rdr: impl BufRead) -> Result<usize> {
    Ok(day1(rdr)?.pop().unwrap_or_default())
}

pub fn part2(rdr: impl BufRead) -> Result<usize> {
    let mut heap = day1(rdr)?;
    let top3 = vec![heap.pop(), heap.pop(), heap.pop()];
    Ok(top3
        .into_iter()
        .filter_map(|x| x)
        .reduce(|acc, x| acc + x)
        .unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_day1() -> Result<()> {
        assert_eq!(part1(Cursor::new("1\n2\n\n3\n4".to_string()))?, 7);
        assert_eq!(
            part2(Cursor::new("1\n2\n\n3\n4\n\n1\n\n5\n6".to_string()))?,
            21
        );
        Ok(())
    }
}
