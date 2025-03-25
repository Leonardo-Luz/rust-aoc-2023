use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::{CaptureMatches, Regex};

fn file_extract(filename: &str) -> Result<Vec<Game>, io::Error> {
    let path = Path::new(filename);

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    let mut games = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        games.push(game_extract(line));
    }

    Ok(games)
}

struct Game {
    _id: i32,
    min_red: i32,
    min_green: i32,
    min_blue: i32,
}

impl Game {
    fn game_power(&self) -> i32 {
        self.min_red * self.min_green * self.min_blue
    }
}

fn color_extract(color_matches: CaptureMatches) -> i32 {
    let mut color = 0;

    for capture in color_matches {
        let color_num = capture.get(1).unwrap().as_str().parse().unwrap();

        color = color.max(color_num);
    }

    color
}

fn game_extract(line: String) -> Game {
    let re = Regex::new(r"Game ([0-9]+): ").unwrap();
    let r_re = Regex::new(r"(\d+) red").unwrap();
    let g_re = Regex::new(r"(\d+) green").unwrap();
    let b_re = Regex::new(r"(\d+) blue").unwrap();

    Game {
        _id: re
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
        min_red: color_extract(r_re.captures_iter(&line)),
        min_green: color_extract(g_re.captures_iter(&line)),
        min_blue: color_extract(b_re.captures_iter(&line)),
    }
}

fn main() {
    let response = file_extract("../data.aoc");

    match response {
        Ok(games) => {
            let mut sum = 0;

            games.iter().for_each(|_game| {
                sum += _game.game_power();
            });

            println!("{}", sum);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
