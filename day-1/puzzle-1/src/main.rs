use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let response = file_extract("./src/data.aoc");

    match response {
        Ok(numbers) => {
            let sum: i32 = numbers.iter().sum();
            println!("Response: {:?}, Sum: {}", numbers, sum);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn file_extract(filename: &str) -> Result<Vec<i32>, io::Error> {
    let path = Path::new(filename);

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    let mut numbers = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        let extracted = extract_numbers(&line);

        if extracted.len() != 2 {
            continue;
        }

        let num = extracted.parse().unwrap();

        numbers.push(num);
    }

    Ok(numbers)
}

fn extract_numbers(text: &str) -> String {
    let response;
    let f_re = Regex::new(r"([0-9]).*([0-9])").unwrap();
    let s_re = Regex::new(r"\d").unwrap();

    match s_re.find(text) {
        Some(mat) => {
            for (_, [start, end]) in f_re.captures_iter(text).map(|c| c.extract()) {
                if start.len() > 0 && end.len() > 0 {
                    response = format!("{}{}", start, end);
                    return response;
                }
            }

            let digit = mat.as_str();
            response = format!("{}{}", digit, digit)
        }
        None => response = "".to_string(),
    }
    response
}
