
use std::num::ParseIntError;
use std::str::FromStr;
use std::{io, path::Path};

macro_rules! parse_err {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        ParseError { message: msg }
    }};
 }

const MAX_NUMBER_OF_CUBES: Sets = Sets {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug)]
struct Games(Vec<Game>);

#[derive(Default, Debug)]
struct Sets {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    cubes: Vec<Sets>,
}

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl From<&str> for ParseError {
    fn from(s: &str) -> Self {
        ParseError {
            message: s.to_string(),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError {
            message: "Failed to parse integer".to_string(),
        }
    }
}

impl FromStr for Sets {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = Self::default();
        for cube in s.split(", ") {
            let (number, color) = cube
                .split_once(" ")
                .ok_or_else(|| parse_err!("cube error: {:?}", cube))?;
            let number = number.parse::<i32>()?;
            cube_set = match color {
                "red" => cube_set.set_red(number),
                "green" => cube_set.set_green(number),
                "blue" => cube_set.set_blue(number),
                _ => return Err(parse_err!("Invalid color: {:?}", color)),
            };
        }
        Ok(cube_set)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl FromStr for Game {
    type Err = ParseError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (game, cubes) = line
            .split_once(": ")
            .ok_or_else(|| ParseError::from("Parse error"))?;
        let id = game
            .split_whitespace()
            .nth(1)
            .and_then(|id| id.trim_end_matches(':').parse::<i32>().ok())
            .ok_or_else(|| parse_err!("Error in {:?}", game))?;
        let set = cubes
            .split("; ")
            .map(Sets::from_str)
            .collect::<Result<Vec<_>, ParseError>>()?;
        Ok(Self { id, cubes: set })
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.cubes.iter().all(Sets::is_possible)
    }
}

impl Games {
    fn parse_to_vec(input: String) -> Self {
        let res = input
            .lines()
            .enumerate()
            .map(|(_, line)| Game::from_str(line))
            .collect::<Result<Vec<Game>, ParseError>>()
            .unwrap_or(vec![]);

        Self(res)
    }

    fn possible_game_ids(&self) -> impl Iterator<Item = i32> + '_ {
        self.0
            .iter()
            .filter(|game| game.is_possible())
            .map(|game| game.id)
    }
}

impl Sets {
    fn set_red(mut self, red: i32) -> Self {
        self.red = red;
        return self;
    }

    fn set_blue(mut self, blue: i32) -> Self {
        self.blue = blue;
        return self;
    }

    fn set_green(mut self, green: i32) -> Self {
        self.green = green;
        return self;
    }

    fn is_possible(&self) -> bool {
        self.red <= MAX_NUMBER_OF_CUBES.red
            && self.green <= MAX_NUMBER_OF_CUBES.green
            && self.blue <= MAX_NUMBER_OF_CUBES.blue
    }
}

pub fn solve_day2() {
    let path = Path::new("./src/day_2/input.txt");
    let text = std::fs::read_to_string(path).unwrap_or("".to_string());
    let t = Games::parse_to_vec(text);

    let r = t.possible_game_ids().sum::<i32>();

    println!("{:?}", r)
}
