use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn get_max_joltage(nums: &Vec<usize>) -> usize {
        // Get max value between 0 and n-1
        // Get max value between max and n
        // return
        let n = nums.len();

        let (max_1_i, max_1) = nums[0..n - 1] // Get the max index and value between 0 and n-1
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_index, value)| value)
            .unwrap();

        let (_max_2_i, max_2) = nums[max_1_i + 1..n]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_index, value)| value)
            .unwrap();
        let answer = max_1 * 10 + max_2;
        // println!("Line: {:?}", nums);
        // println!("Answer: {answer}");
        answer
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            let nums = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>();
            answer += get_max_joltage(&nums);
        }
        Ok(answer)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn get_max_joltage_2(nums: &Vec<usize>) -> usize {
        //Get the first max value between 0 and n-12
        //Get max value between prev_max_i and n-11
        //Get max value between prev_max_i and n-10
        //Repeat till the last digit

        let nums_enumerated = nums.iter().enumerate().collect::<Vec<_>>();

        println!("Line: {:?}", nums);

        let n = nums.len();
        let mut prev_max_i = 0;
        let mut final_val = 0;
        let initial_offset = 11;
        let mut power = 10usize.pow(11);

        for i in (0..=initial_offset).rev() {
            let slice = &nums_enumerated[prev_max_i..n - i];
            println!("Slice: {:?}", slice);
            let (max_index, max_value) = slice.iter().rev().max_by_key(|&(_index, value)| value).unwrap();
            println!("Max index: {max_index}, Max value: {max_value}");
            prev_max_i = max_index + 1;
            final_val += power * **max_value;
            power /= 10;
        }
        println!("Answer: {final_val}");

        final_val
        //return final val
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            let nums = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>();
            answer += get_max_joltage_2(&nums);
        }
        Ok(answer)    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}


