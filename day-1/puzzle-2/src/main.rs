use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let response = file_extract("../data.aoc");
    match response {
        Ok(numbers) => {
            let sum: i32 = numbers.iter().sum();
            println!("Response: {:?},\nSum: {}", numbers, sum);
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

fn extract_writed_numbers(text: &str) -> String {
    let f_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let s_re = Regex::new(r"(2ne|7ine|9ight|5ight|1ight|3ight|8wo|8hree)").unwrap();

    let mut response = f_re
        .replace_all(text, |caps: &regex::Captures| match &caps[1] {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => "",
        })
        .to_string();

    response = s_re
        .replace_all(&response, |caps: &regex::Captures| match &caps[1] {
            "2ne" => "21",
            "7ine" => "79",
            "9ight" => "98",
            "5ight" => "58",
            "1ight" => "18",
            "3ight" => "38",
            "8wo" => "82",
            "8hree" => "83",
            _ => "",
        })
        .to_string();

    s_re.replace_all(&response, |caps: &regex::Captures| match &caps[1] {
        "2ne" => "21",
        "7ine" => "79",
        "9ight" => "98",
        "5ight" => "58",
        "1ight" => "18",
        "3ight" => "38",
        "8wo" => "82",
        "8hree" => "83",
        _ => "",
    })
    .to_string()
}

fn extract_numbers(text: &str) -> String {
    let response;
    let f_re = Regex::new(r"([0-9]).*([0-9])").unwrap();
    let s_re = Regex::new(r"\d").unwrap();

    let parsed_text = extract_writed_numbers(text);

    match s_re.find(&parsed_text) {
        Some(mat) => {
            for (_, [start, end]) in f_re.captures_iter(&parsed_text).map(|c| c.extract()) {
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
