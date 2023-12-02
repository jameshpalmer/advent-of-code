use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;

    for l in reader.lines() {
        let line = l?;
        let mut maybe_first_row_num: Option<char> = None;
        let mut maybe_last_row_num: Option<char> = None;

        for c in line.chars() {
            if !c.is_digit(10) {
                continue;
            }
            match maybe_first_row_num {
                Some(_) => {}
                None => maybe_first_row_num = Some(c),
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

    println!("Part 1: {}", total);

    Ok(())
}
