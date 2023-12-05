use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let line_count = lines.len();
    let mut copies = vec![1; line_count];

    for (i, scratchcard) in lines.into_iter().enumerate() {
        let scratchcard_nums: Vec<&str> = scratchcard.split(":").collect();

        if let [winners_str, nums_str] = scratchcard_nums[1].split("|").collect::<Vec<_>>()[..] {
            let winners: Vec<_> = winners_str
                .split_whitespace()
                .map(|winner_str| winner_str.parse::<i32>().unwrap())
                .collect();

            let nums: Vec<_> = nums_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect();

            let winner_count: i32 = nums
                .iter()
                .fold(0, |acc, num| acc + winners.contains(num) as i32);

            for scratchcard_index in i + 1..i + 1 + winner_count as usize {
                copies[scratchcard_index] += copies[i];
            }
        }
    }

    println!("{:?}", copies.iter().sum::<i32>());
    Ok(())
}
