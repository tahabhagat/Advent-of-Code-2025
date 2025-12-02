use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn id_is_invalid(id: u64) -> bool {
        let num_digits = match id {
            0 => 1,
             _ => id.ilog10() +1 ,
        };
        if num_digits % 2 != 0 { // numbers with odd number of digits cannot be invalid
            return false;
        }
        let half = num_digits / 2;
        let ten_to_digits = 10u64.pow(half);
        if id % ten_to_digits == id / ten_to_digits {
            return true;
        }
        false
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<u64> {
        let mut answer = 0;
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        for range in input.split(',') {
            let start = range.split('-').next().expect(&"Failed to unwrap start".to_string()).parse::<u64>()?;
            let end = range.split('-').last().expect(&"Failed to unwrap end".to_string()).parse::<u64>()?;

            for i in start..=end {
                if id_is_invalid(i) {
                    answer += i;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn id_is_invalid_part2(id: u64) -> bool {


        let id = id.to_string();
        let len = id.len();
        for size in 1..=len/2 {
            if len % size == 0 {
                let pattern = &id[..size];
                if pattern.repeat(len/size) == id {
                    return true;
                }
            }
        }
        false
    }
    fn part2<R: BufRead>(mut reader: R) -> Result<u64> {
        let mut answer = 0;
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        for range in input.split(',') {
            let start = range.split('-').next().unwrap().parse::<u64>()?;
            let end = range.split('-').last().unwrap().parse::<u64>()?;
            for i in start..=end {
                if id_is_invalid_part2(i) {
                    answer += i;
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
