use std::cmp::max;
use std::fs::read_to_string;

const NUM_RED: u32 = 12;
const NUM_BLUE: u32 = 14;
const NUM_GREEN: u32 = 13;

#[derive(Debug)]
struct Game {
    max_blue: u32,
    max_red: u32,
    max_green: u32,
    pub game_number: u32,
}

impl Game {
    pub fn new(game_number: u32, max_blue: u32, max_red: u32, max_green: u32) -> Self {
        Self {
            max_blue,
            max_red,
            max_green,
            game_number,
        }
    }

    pub fn is_possible(&self) -> bool {
        if self.max_red > NUM_RED || self.max_blue > NUM_BLUE || self.max_green > NUM_GREEN {
            return false;
        }

        true
    }
}

#[derive(Debug)]
struct SetResult {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

fn main() {
    let mut all_games: Vec<Game> = vec![];
    let mut possible_games: Vec<Game> = vec![];
    let game_input = read_lines();

    let mut sum: u32 = 0;

    for input in &game_input {
        all_games.push(game_process(input));
    }

    for game in all_games {
        if game.is_possible() {
            possible_games.push(game);
        }
    }

    for game in possible_games {
        sum += game.game_number;
    }

    println!("{}", sum);
}

fn game_process(input: &String) -> Game {
    let mut max_red: u32 = 0;
    let mut max_blue: u32 = 0;
    let mut max_green: u32 = 0;
    let mut set_results: Vec<SetResult> = vec![];
    let x: Vec<&str> = input.split_terminator(&[':', ';'][..]).collect();

    let game_number = get_game_number(x[0].to_string());

    for i in 1..x.len() {
        set_results.push(calculate_set(x[i]));
    }

    for res in set_results {
        max_red = max(max_red, res.red);
        max_blue = max(max_blue, res.blue);
        max_green = max(max_green, res.green);
    }

    Game::new(game_number, max_blue, max_red, max_green)
}

fn get_game_number(game_input: String) -> u32 {
    let mut game_number_str: String = String::new();

    let game_number: Vec<&str> = game_input.matches(char::is_numeric).collect();

    for num in game_number {
        game_number_str.push_str(num);
    }

    game_number_str.parse::<u32>().unwrap()
}

fn calculate_set(input: &str) -> SetResult {
    let mut green: u32 = 0;
    let mut red: u32 = 0;
    let mut blue: u32 = 0;
    let x: Vec<&str> = input.split_terminator(',').collect();

    for color in x {
        let mut colors_iter = color.split_whitespace();

        let number_str = colors_iter.next().unwrap();
        let color = colors_iter.next().unwrap();
        let number = number_str.parse::<u32>().unwrap();

        match color {
            "red" => red = number,
            "green" => green = number,
            "blue" => blue = number,
            _ => {
                panic!()
            }
        }
    }

    SetResult { blue, red, green }
}

fn read_lines() -> Vec<String> {
    read_to_string("day2-2/input_file.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
