use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() {
    let _response = file_extract("../data.aoc");
}

fn file_extract(filename: &str) -> Result<Vec<i32>, io::Error> {
    let path = Path::new(filename);

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    let mut nums = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        nums.push(number_extract(line));
    }

    Ok(nums)
}

fn number_extract(line: String) -> i32 {
    let re = Regex::new(r"[^[0-9].]").unwrap();

    // implement double linked list
    let _response = match re.find(&line) {
        Some(_mat) => {
            // line.before
            // line.after
            return 0;
        }
        None => 0,
    };

    0
}
