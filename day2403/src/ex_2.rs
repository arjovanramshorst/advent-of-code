use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    re.captures_iter(input).fold(0, |acc, captures| {
        let full = captures[0].to_string();
        return if full.eq("do()") {
            enabled = true;
            acc
        } else if full.eq("don't()") {
            enabled = false;
            acc
        } else if !enabled {
            acc
        } else {
            acc + captures[1].parse::<i32>().unwrap() * captures[2].parse::<i32>().unwrap()
        };
    })
}
