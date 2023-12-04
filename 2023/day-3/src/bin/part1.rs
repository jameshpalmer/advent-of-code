use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut line_symbol_locs: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, l) in reader.lines().enumerate() {
        let line = l?;

        let locs: Vec<usize> = line
            .char_indices()
            .filter(|&(_, c)| !c.is_digit(10) && c != '.')
            .map(|(i, _)| i)
            .collect();

        line_symbol_locs.insert(i, locs);
    }

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for (i, l) in reader.lines().enumerate() {
        let line = l?;

        let mut indices_and_nums: Vec<(usize, &str)> = Vec::new();
        let mut current_number_start: Option<usize> = None;

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if current_number_start.is_none() {
                    current_number_start = Some(i);
                }
            } else if let Some(start) = current_number_start {
                let num_str = &line[start..i];
                indices_and_nums.push((start, num_str));
                current_number_start = None;
            }
        }

        // Check for number at end of string
        if let Some(start) = current_number_start {
            let number = &line[start..];
            indices_and_nums.push((start, number));
        }

        for (start_index, num_str) in indices_and_nums {
            // Check line above
            if (i > 0
                && line_symbol_locs[&(i - 1)].iter().any(|&number| {
                    let adjusted_start_index = start_index.checked_sub(1).unwrap_or(0);
                    let end_index = start_index + num_str.len();

                    number >= adjusted_start_index && number <= end_index
                }))
                || line_symbol_locs[&i].iter().any(|&number| {
                    let adjusted_start_index = start_index.checked_sub(1).unwrap_or(0);
                    let end_index = start_index + num_str.len();

                    number == adjusted_start_index || number == end_index
                })
                || (line_symbol_locs.contains_key(&(i + 1))
                    && line_symbol_locs[&(i + 1)].iter().any(|&number| {
                        let adjusted_start_index = start_index.checked_sub(1).unwrap_or(0);
                        let end_index = start_index + num_str.len();

                        number >= adjusted_start_index && number <= end_index
                    }))
            {
                sum += num_str.parse::<u32>().unwrap_or(0);
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
