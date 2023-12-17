use std::cmp::max;
use std::ops::Add;

struct Game {
    id: u32,
    draws: Vec<Draw>,
}
impl Game {
    fn max(self) -> (u32, u32, u32) {
        self.draws
            .iter()
            .map(|draw| (draw.red, draw.green, draw.blue))
            .fold((0, 0, 0), |(ar, ag, ab), (r, g, b)| {
                (max(ar, r), max(ag, g), max(ab, b))
            })
    }
}

#[derive(Clone, Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}
impl Add for Draw {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Draw {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

fn parse_cube(raw: &str) -> Draw {
    let mut vals = raw.split(' ');
    let (num, color) = (
        vals.next()
            .expect("Format Violation")
            .parse::<u32>()
            .expect("Format Violation"),
        vals.next().expect("Format Violation"),
    );
    let mut out = Draw::default();
    match color {
        "red" => out.red = num,
        "green" => out.green = num,
        "blue" => out.blue = num,
        _ => panic!("Format Violation"),
    };
    out
}

fn parse_draw(raw: &str) -> Draw {
    raw.split(", ")
        .map(parse_cube)
        .fold(Draw::default(), |acc, draw| acc + draw)
}

fn parse_input(input: &str) -> Game {
    let mut split = input.split(": ");
    let id: u32 = split
        .next()
        .expect("Format Violation")
        .split(' ')
        .last()
        .expect("Format Violation")
        .parse()
        .unwrap();

    let draws: Vec<Draw> = split
        .next()
        .expect("Format Violation")
        .split("; ")
        .map(parse_draw)
        .collect();

    Game { id, draws }
}

fn check_possible(game: Game, cubes: [u32; 3]) -> Option<u32> {
    let result: Vec<Draw> = game
        .draws
        .iter()
        .filter(|draw| draw.red > cubes[0] || draw.green > cubes[1] || draw.blue > cubes[2])
        .cloned()
        .collect();

    if result.is_empty() {
        Some(game.id)
    } else {
        None
    }
}

pub mod p1 {
    const EXISTING_CUBES: [u32; 3] = [12, 13, 14];
    pub fn validate_games(input: &str) -> u32 {
        input
            .lines()
            .map(super::parse_input)
            .filter_map(|game| super::check_possible(game, EXISTING_CUBES))
            .sum()
    }
}

pub mod p2 {
    pub fn sum_powers(input: &str) -> u32 {
        input
            .lines()
            .map(super::parse_input)
            .map(|game| game.max())
            .map(|(r, g, b)| r * g * b)
            .sum()
    }
}
