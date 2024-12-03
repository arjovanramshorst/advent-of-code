use lib::NrString;
use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let vec: Vec<i64> = input
        .lines()
        .map(|line| {
            re.captures(line.replace(" ", "").as_str())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap()
        })
        .collect();

    let time = vec[0];
    let record = vec[1];

    let matches = (0..time)
        .filter(|hold| (time - hold) * hold > record)
        .count();
    return matches as i32;
}
