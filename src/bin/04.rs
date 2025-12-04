use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");



    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let offset_list = vec![
            (-1,-1), (-1,0), (-1,1),
            (0,-1),          (0,1),
            (1,-1), (1,0), (1,1),
        ];

        let lines  = reader.lines().map(
            |line|
                line.unwrap().chars().map(
                |c|
                    c == '@'
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

        // println!("{:?}", lines);

        let n = lines.len();
        let m = lines[0].len();

        let mut count = 0;

        for i in 0..n {
            for j in 0..m {
                if(!lines[i][j]){
                    continue
                }
                // println!("Found @ at {i}, {j}");
                let mut count_adjacent = 0;
                for (dx, dy) in &offset_list {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
                        if lines[x as usize][y as usize] {
                            count_adjacent += 1;
                        }
                    }
                }
                // println!("Adjacent = {}", count_adjacent);
                if count_adjacent < 4 {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let offset_list = vec![
            (-1,-1), (-1,0), (-1,1),
            (0,-1),          (0,1),
            (1,-1), (1,0), (1,1),
        ];

        let mut lines  = reader.lines().map(
            |line|
                line.unwrap().chars().map(
                    |c|
                        c == '@'
                ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

        // println!("{:?}", lines);

        let n = lines.len();
        let m = lines[0].len();

        let mut count = 0;
        let mut prev_count = 1;

        while count != prev_count {
            prev_count = count;
            for i in 0..n {
                for j in 0..m {
                    if (!lines[i][j]) {
                        continue;
                    }
                    // println!("Found @ at {i}, {j}");
                    let mut count_adjacent = 0;
                    for (dx, dy) in &offset_list {
                        let x = i as i32 + dx;
                        let y = j as i32 + dy;
                        if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
                            if lines[x as usize][y as usize] {
                                count_adjacent += 1;
                            }
                        }
                    }
                    // println!("Adjacent = {}", count_adjacent);
                    if count_adjacent < 4 {
                        lines[i][j] = false; // Remove the roll from the board
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
