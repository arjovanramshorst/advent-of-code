use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;
use std::ops::Range;

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

    let mut gear_map: HashMap<XY, Vec<usize>> = HashMap::new();

    let re = Regex::new("([0-9]+)").unwrap();
    input
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
        .for_each(|part_number| {
            get_part_locations(&part_number, width, height)
                .iter()
                .for_each(|xy| {
                    let char = character_matrix.get(xy.y).unwrap().get(xy.x).unwrap();
                    if char.to_string() == "*" {
                        println!("Found a gear: {:?}, id: {:?}", xy, part_number);
                        &gear_map
                            .entry(*xy)
                            .or_insert_with(Vec::new)
                            .push(part_number.id);
                    }
                });
        });

    let res = gear_map.iter().map(|it| it.1);
    let res = res.filter(|it| it.iter().count() == 2);
    let res = res.map(|it| it[0] * it[1]).reduce(|a, b| a + b);

    return res.unwrap().to_string();
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
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
