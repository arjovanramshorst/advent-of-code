use regex::Regex;
use std::cmp::min;
use std::collections::HashSet;

pub fn solve(input: &str) -> String {
    let line_count = input.lines().count();
    let mut card_count = Vec::from_iter((0..line_count).map(|it| 1));
    input.lines().enumerate().for_each(|(index, line)| {
        let score = parse_line(line) as usize;
        let current_card_count = card_count[index].clone();
        let copy_range = (index + 1)..=min(index + score, line_count - 1);
        copy_range.for_each(|it| card_count[it] += current_card_count);
    });

    let sum = card_count.iter().sum::<usize>().to_string();

    return format!("{}", sum); // sum.unwrap());
}

fn parse_line(line: &str) -> u32 {
    let re_first = Regex::new(r"Card +[0-9]+: ([0-9\s]+)").unwrap();
    let re_second = Regex::new(r"\| +([0-9\s]+)").unwrap();

    let winning_numbers = re_first.captures(line).unwrap();
    // dbg!(&winning_numbers);
    let winning_numbers = winning_numbers
        .get(1)
        .unwrap()
        .as_str()
        .trim()
        .split_whitespace()
        .map(|nr| nr.parse::<u32>().unwrap());
    let own_numbers = re_second.captures(line).unwrap();
    // dbg!(&own_numbers);
    let own_numbers = own_numbers
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|nr| nr.parse::<u32>().unwrap());
    let winning_set: HashSet<u32> = HashSet::from_iter(winning_numbers);
    let own_numbers = HashSet::from_iter(own_numbers);

    return winning_set.intersection(&own_numbers).count() as u32;
}
