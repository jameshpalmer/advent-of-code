use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total: u32 = 0;

    for l in reader.lines() {
        let game = l?;
        let maybe_result = game.splitn(2, ": ").nth(1);
        match maybe_result {
            Some(value) => {
                total += get_power(value);
            }
            None => eprintln!("Error: Unable to extract value from line: {}", game),
        }
    }

    println!("{}", total);

    Ok(())
}

fn get_power(game: &str) -> u32 {
    let counts: Vec<&str> = game
        .split("; ")
        .flat_map(|substr| substr.split(", "))
        .collect();

    let mut max: HashMap<&str, u32> = HashMap::new();
    max.insert("red", 0);
    max.insert("green", 0);
    max.insert("blue", 0);

    for count in counts {
        let mut words = count.split_whitespace();

        if let Some(str_num) = words.next() {
            if let Some(color) = words.next() {
                let num: u32 = str_num
                    .parse()
                    .expect("Failed to parse input string as i32");

                if num > max[color] {
                    max.insert(color, num);
                }
            }
        }
    }
    return max["red"] * max["green"] * max["blue"];
}
