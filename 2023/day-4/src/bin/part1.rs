use std::cmp::max;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut points = 0;

    for line in reader.lines() {
        let scratchcard: String = line?;
        let scratchcard_nums: Vec<&str> = scratchcard.split(":").collect();

        if let [winners_str, nums_str] = scratchcard_nums[1].split("|").collect::<Vec<_>>()[..] {
            let winners: Vec<_> = winners_str
                .split_whitespace()
                .map(|winner_str| winner_str.parse::<u32>().unwrap())
                .collect();

            let nums: Vec<_> = nums_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect();

            let winner_sum: i32 = nums.iter().fold(0, |acc, num| {
                if winners.contains(num) {
                    max(acc * 2, 1)
                } else {
                    acc
                }
            });

            points += winner_sum;
        }
    }
    println!("{}", points);
    Ok(())
}
