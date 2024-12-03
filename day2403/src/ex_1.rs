use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input).fold(0, |acc, captures| {
        return acc + captures[1].parse::<i32>().unwrap() * captures[2].parse::<i32>().unwrap();
    })
}
