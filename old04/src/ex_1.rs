use regex::Regex;
use std::collections::HashSet;

pub fn solve(input: &str) -> String {
    let re_first = Regex::new(r"Card +[0-9]+: ([0-9\s]+)").unwrap();
    let re_second = Regex::new(r"\| +([0-9\s]+)").unwrap();

    let sum = input
        .lines()
        .map(|line| {
            dbg!(&line);
            let winning_numbers = re_first.captures(line).unwrap();
            dbg!(&winning_numbers);
            let winning_numbers = winning_numbers
                .get(1)
                .unwrap()
                .as_str()
                .trim()
                .split(" ")
                .filter(|it| !it.is_empty())
                .map(|nr| nr.parse::<u32>().unwrap());
            let own_numbers = re_second.captures(line).unwrap();
            // dbg!(&own_numbers);
            let own_numbers = own_numbers
                .get(1)
                .unwrap()
                .as_str()
                .trim()
                .split(" ")
                .filter(|it| !it.is_empty())
                .map(|nr| nr.parse::<u32>().unwrap());
            let winning_set: HashSet<u32> = HashSet::from_iter(winning_numbers);
            let own_numbers = HashSet::from_iter(own_numbers);

            let counts = winning_set.intersection(&own_numbers).count();

            return match counts {
                0 => 0,
                counts => 2_u32.checked_pow(counts as u32 - 1).unwrap(),
            };
        })
        .reduce(|a, b| a + b);

    return format!("{}", sum.unwrap());
}
