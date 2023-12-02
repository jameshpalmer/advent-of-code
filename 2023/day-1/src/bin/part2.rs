use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let substitutions = [
        ("oneight", "18"),
        ("twone", "21"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("sevenine", "79"),
        ("eightwo", "82"),
        ("eighthree", "82"),
        ("nineight", "98"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut total = 0;

    for l in reader.lines() {
        let mut line = l?;

        for pair in substitutions {
            line = line.replace(pair.0, pair.1);
        }

        let mut maybe_first_row_num: Option<char> = None;
        let mut maybe_last_row_num: Option<char> = None;

        for c in line.chars() {
            if !c.is_digit(10) {
                continue;
            }
            match maybe_first_row_num {
                None => maybe_first_row_num = Some(c),
                Some(_) => {}
            }
            maybe_last_row_num = Some(c);
        }

        let first_row_num = maybe_first_row_num
            .ok_or_else(|| Error::new(ErrorKind::Other, "Error: Row has no first number"))?;
        let last_row_num = maybe_last_row_num
            .ok_or_else(|| Error::new(ErrorKind::Other, "Error: Row has no last number"))?;

        let string_num = vec![first_row_num, last_row_num].iter().collect::<String>();
        let parsed_num: i32 = string_num
            .parse()
            .expect("Failed to parse input string as i32");

        total += parsed_num;
    }

    println!("{}", total);

    Ok(())
}
