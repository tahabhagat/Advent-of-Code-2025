use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        let mut counter = 0;
        let mut current_pos = 50;
        for line in lines {
            let line = line?; // Unwrap the line to get the string

            let distance = match line.chars().next() { // Get the distance and direction from line
                Some('L') => -line[1..].parse::<isize>()?,
                Some('R') => line[1..].parse::<isize>()?,
                _ => panic!("Invalid direction"),
            };

            current_pos = get_next_pos(current_pos, distance); // Update current pos with the next pos

            if current_pos == 0 { // If pos is 0, increment counter
                counter += 1;
            }

        }

        Ok(counter)
    }

    fn get_next_pos(current_pos: isize, distance: isize) -> isize {

        let mut normalised_distance = distance;


        if distance >= 100 || distance <= -100 { // If the distance is greater than 100, we can just take the remainder
             normalised_distance = distance % 100;
        }
        if normalised_distance < 0 { // If distance is negative, we add 100 to it to make it positive
            normalised_distance += 100;
        }
        let next_pos = current_pos + normalised_distance;

        // The next pos can now > 100, in which case we return the remainder
        if next_pos >= 100 {
            return next_pos % 100;
        }
        //Else we can return the next pos
        next_pos
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        let mut counter = 0;
        let mut current_pos = 50;

        for line in lines {
            let line = line?; // Unwrap the line to get the string
            let distance = match line.chars().next() { // Get the distance and direction from line
                Some('L') => -line[1..].parse::<isize>()?,
                Some('R') => line[1..].parse::<isize>()?,
                _ => panic!("Invalid direction"),
            };
            let (new_pos, zeroes_traversed) = get_next_zeroes(current_pos, distance);
            current_pos = new_pos;
            counter += zeroes_traversed;
        }

        Ok(counter)
    }

    /*
    Get the number of zeroes traversed given current pos and distance
     */
    fn get_next_zeroes(current_pos: isize, distance: isize) -> (isize, usize) {
        let mut zeroes_traversed: usize = 0;
        let mut normalised_distance = distance;
        if normalised_distance >= 100 || normalised_distance <= -100 {
            zeroes_traversed += (normalised_distance / 100).abs() as usize; // for every full loop, add 1 to the zeroes traversed
            normalised_distance = normalised_distance % 100; // get the remaining clicks separately
        }
        let mut next_pos = current_pos + normalised_distance;

        match next_pos {
            0 | 100 => {
                zeroes_traversed += 1;
                next_pos = 0;
            }
            n if n < 0 && n > -100 => {
                next_pos += 100;
                if current_pos != 0 {
                    zeroes_traversed+=1
                }
            }
            n if n > 0 && n < 100 => {},
            n if n < -100 => {
                // Add the extra clicks to zeroes traversed
                zeroes_traversed += (next_pos / 100).abs() as usize;
                //get the remainder
                next_pos = (next_pos % 100) + 100;
            }
            n if n > 100 => {
                zeroes_traversed += (next_pos/100).abs() as usize;
                next_pos = next_pos % 100;
            }
            _ => panic!("Invalid position"),
        }
        (next_pos, zeroes_traversed)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
