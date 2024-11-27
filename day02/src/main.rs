use regex::Regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution: {}", solve(input));
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

#[derive(Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

fn solve(input: &str) -> String {
    let games: Vec<Game> = input
        .lines()
        .map(|l| {
            let id = parse_number("Game ([0-9]+)", l);
            let pulls: Vec<Pull> = l
                .split(":")
                .nth(1)
                .unwrap()
                .split(";")
                .map(|game| Pull {
                    red: parse_number("([0-9]+) red", game),
                    green: parse_number("([0-9]+) green", game),
                    blue: parse_number("([0-9]+) blue", game),
                })
                .collect();
            let game = Game { id, pulls };
            // println!("{}", l);
            // println!("{:?}", game);
            return game;
        })
        .collect();
    let res1 = games.iter().filter(|g| {
        let invalid = g.pulls.iter().any(|p| p.red > MAX_RED)
            || g.pulls.iter().any(|p| p.green > MAX_GREEN)
            || g.pulls.iter().any(|p| p.blue > MAX_BLUE);
        return !invalid;
    });

    let res1 = res1.map(|g| g.id).reduce(|a, b| a + b).unwrap();
    println!("Solution 1: {}", res1);

    let res2 = games
        .iter()
        .map(|g| {
            let min_red = g.pulls.iter().map(|p| p.red).max().unwrap_or(0);
            let min_green = g.pulls.iter().map(|p| p.green).max().unwrap_or(0);
            let min_blue = g.pulls.iter().map(|p| p.blue).max().unwrap_or(0);
            return min_red * min_green * min_blue;
        })
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Solution 2: {}", res2);

    return res1.to_string();
}

fn parse_number(regex: &str, line: &str) -> u32 {
    let matched = Regex::new(regex).unwrap().captures(line);
    let matched = match matched {
        Some(val) => val.get(1),
        None => return 0,
    };
    match matched {
        Some(val) => val.as_str().parse::<u32>().unwrap(),
        None => 0,
    }
}
