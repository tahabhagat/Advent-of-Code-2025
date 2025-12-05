use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut count = 0usize;

        let lines = reader.lines();
        let mut ranges_completed = false;

        // Create a set of tuples?

        let mut ranges: Vec<(usize, usize)> = Vec::new();
        let mut values: Vec<usize> = Vec::new();

        for line in lines {
            let line = line?;
            if ranges_completed {
                let num = line.parse::<usize>()?;
                // println!("Checking {num}");
                values.push(num);
            } else if line == "" {
                // Reached blank line, transition to inputs
                // println!("Set :{:?}", fresh_set);
                ranges_completed = true;
            } else {
                let start = line.split('-').next().unwrap().parse::<usize>()?;
                let end = line.split('-').last().unwrap().parse::<usize>()?;
                ranges.push((start, end));
            }
        }

        ranges.sort_by_key(|i| i.0);
        let mut merged = Vec::new();
        let mut current = ranges[0];

        for &(start, end) in &ranges[1..] {
            if start <= current.1 {
                current.1 = end.max(current.1);
            } else {
                merged.push(current);
                current = (start, end);
            }
        }
        merged.push(current); // Merged ranges are now ready

        // println!("Merged ranges: {:?}", merged);
        for value in values {
            for range in &merged {
                if value >= range.0 && value <= range.1 {
                    // println!("Found match for {value}");
                    count += 1;
                    break;
                } else if value < range.0 {
                    break;
                }
            }
        }

        Ok(count)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut ranges: Vec<(usize, usize)> = Vec::new();
        let mut count = 0usize;
        for line in reader.lines() {
            let line = line?;

            if line == "" {
                // Reached blank line, move to next part
                break;
            }

            let start = line.split('-').next().unwrap().parse::<usize>()?;
            let end = line.split('-').last().unwrap().parse::<usize>()?;
            ranges.push((start, end));
        }

        ranges.sort_by_key(|i| i.0);
        let mut merged = Vec::new();
        let mut current = ranges[0];

        for &(start, end) in &ranges[1..] {
            if start <= current.1 {
                current.1 = end.max(current.1);
            } else {
                merged.push(current);
                current = (start, end);
            }
        }
        merged.push(current); // Merged ranges are now ready

        for range in &merged {
            count += range.1 - range.0 + 1;
        }

        Ok(count)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
