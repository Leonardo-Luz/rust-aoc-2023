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
    id: i32,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
}

impl Game {
    fn game_is_possible(&self) -> bool {
        self.max_red <= MAX_RED && self.max_green <= MAX_GREEN && self.max_blue <= MAX_BLUE
    }
}

fn color_extract(color_matches: CaptureMatches) -> i32 {
    let mut max_color = 0;

    for capture in color_matches {
        let color_num = capture.get(1).unwrap().as_str().parse().unwrap();

        max_color = max_color.max(color_num);
    }

    max_color
}

fn game_extract(line: String) -> Game {
    let re = Regex::new(r"Game ([0-9]+): ").unwrap();
    let r_re = Regex::new(r"(\d+) red").unwrap();
    let g_re = Regex::new(r"(\d+) green").unwrap();
    let b_re = Regex::new(r"(\d+) blue").unwrap();

    Game {
        id: re
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
        max_red: color_extract(r_re.captures_iter(&line)),
        max_green: color_extract(g_re.captures_iter(&line)),
        max_blue: color_extract(b_re.captures_iter(&line)),
    }
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let response = file_extract("../data.aoc");

    match response {
        Ok(games) => {
            let mut sum = 0;

            games.iter().for_each(|_game| {
                if _game.game_is_possible() {
                    sum += _game.id
                }
            });

            println!("{}", sum);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
