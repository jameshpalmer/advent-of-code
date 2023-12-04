use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut line_indices_and_nums: HashMap<usize, Vec<(usize, String)>> = HashMap::new();

    for (i, l) in reader.lines().enumerate() {
        let line = l?;

        let mut indices_and_nums: Vec<(usize, String)> = Vec::new();
        let mut current_number_start: Option<usize> = None;

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if current_number_start.is_none() {
                    current_number_start = Some(i);
                }
            } else if let Some(start) = current_number_start {
                let num_str = line[start..i].to_string();
                indices_and_nums.push((start, num_str));
                current_number_start = None;
            }
        }

        // Check for number at the end of the string
        if let Some(start) = current_number_start {
            let number = line[start..].to_string();
            indices_and_nums.push((start, number));
        }

        line_indices_and_nums.insert(i, indices_and_nums);
    }

    let mut sum = 0;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for (i, l) in reader.lines().enumerate() {
        let line = l?;

        let symbol_locs: Vec<usize> = line
            .char_indices()
            .filter(|&(_, c)| !c.is_digit(10) && c != '.')
            .map(|(i, _)| i)
            .collect();

        'symbol_loop: for symbol_index in symbol_locs {
            let mut count_parts: u32 = 0;
            let mut gear_ratio: u32 = 1;

            if i > 0 {
                for (start_index, str_num) in line_indices_and_nums[&(i - 1)].clone() {
                    if symbol_index >= start_index.checked_sub(1).unwrap_or(0)
                        && symbol_index <= start_index + str_num.len()
                    {
                        if count_parts > 1 {
                            continue 'symbol_loop;
                        }

                        count_parts += 1;
                        gear_ratio *= str_num.parse::<u32>().unwrap_or(1)
                    }
                }
            }

            for (start_index, str_num) in line_indices_and_nums[&i].clone() {
                if symbol_index == start_index.checked_sub(1).unwrap_or(0)
                    || symbol_index == start_index + str_num.len()
                {
                    if count_parts > 1 {
                        continue 'symbol_loop;
                    }

                    count_parts += 1;
                    gear_ratio *= str_num.parse::<u32>().unwrap_or(1)
                }
            }

            if line_indices_and_nums.contains_key(&(i + 1)) {
                for (start_index, str_num) in line_indices_and_nums[&(i + 1)].clone() {
                    if symbol_index >= start_index.checked_sub(1).unwrap_or(0)
                        && symbol_index <= start_index + str_num.len()
                    {
                        if count_parts > 1 {
                            continue 'symbol_loop;
                        }

                        count_parts += 1;
                        gear_ratio *= str_num.parse::<u32>().unwrap_or(1)
                    }
                }
            }

            if count_parts == 2 {
                sum += gear_ratio;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
