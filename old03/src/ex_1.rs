use regex::Regex;
use std::cmp::min;
use std::ops::Range;

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution: {}", solve(input));
}

#[derive(Debug)]
struct PartNumber {
    id: usize,
    line: usize,
    range: Range<usize>,
}

pub fn solve(input: &str) -> String {
    let character_matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = character_matrix.len();
    let width = character_matrix.get(0).unwrap().len();

    println!("height: {}, width: {}", height, width);

    let re = Regex::new("([0-9]+)").unwrap();
    let part_numbers: Vec<PartNumber> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            re.captures_iter(line).map(move |m| PartNumber {
                id: m.get(1).unwrap().as_str().parse().unwrap(),
                line: y,
                range: m.get(1).unwrap().range(),
            })
        })
        .flatten()
        .filter(|part_number| {
            let part_locations = get_part_locations(part_number, width, height);
            let is_valid = part_locations.iter().any(|xy| {
                let char = character_matrix.get(xy.y).unwrap().get(xy.x).unwrap();
                return !char.is_numeric() && char.to_string() != ".";
            });
            return is_valid;
        })
        .collect();

    let sum = part_numbers
        .iter()
        .map(|pn| pn.id)
        .reduce(|a, b| a + b)
        .unwrap();

    return sum.to_string();
}

#[derive(Debug)]
struct XY {
    x: usize,
    y: usize,
}

fn get_part_locations(part_number: &PartNumber, width: usize, height: usize) -> Vec<XY> {
    let y_range = if part_number.line == 0 {
        0..=1
    } else if part_number.line == height - 1 {
        (part_number.line - 1)..=(part_number.line)
    } else {
        (part_number.line - 1)..=(part_number.line + 1)
    };

    let x_range =
        part_number.range.start.checked_sub(1).unwrap_or(0)..=min(part_number.range.end, width - 1);

    let res = y_range
        .map(move |y| x_range.clone().map(move |x| XY { x: x, y: y }))
        .flatten()
        .collect();

    return res;
}
