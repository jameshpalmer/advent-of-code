use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static COLOR_LIMITS: Lazy<HashMap<&'static str, u32>> = Lazy::new(|| {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("red", 12);
    map.insert("green", 13);
    map.insert("blue", 14);
    return map;
});

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total: usize = 0;

    for (i, l) in reader.lines().enumerate() {
        let game = l?;
        let maybe_result = game.splitn(2, ": ").nth(1);
        match maybe_result {
            Some(value) => {
                if is_possible_game(value) {
                    total += i + 1;
                }
            }
            None => eprintln!("Error: Unable to extract value from line: {}", game),
        }
    }

    println!("{}", total);

    Ok(())
}

fn is_possible_game(game: &str) -> bool {
    let counts: Vec<&str> = game
        .split("; ")
        .flat_map(|substr| substr.split(", "))
        .collect();

    for count in counts {
        let mut words = count.split_whitespace();

        if let Some(str_num) = words.next() {
            if let Some(color) = words.next() {
                let num: u32 = str_num
                    .parse()
                    .expect("Failed to parse input string as i32");
                if num > COLOR_LIMITS[color] {
                    return false;
                }
            }
        }
    }
    return true;
}
