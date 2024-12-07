use lib::*;

fn pipe(left: u64, right: u64) -> u64 {
    return format!("{}{}", left.to_string(), right.to_string())
        .parse()
        .unwrap();
}

fn check_recursive(check: u64, agg: u64, rest: &[u64]) -> bool {
    let val = rest[0];
    if rest.iter().count() == 1 {
        agg * val == check || agg + val == check || pipe(agg, val) == check
    } else {
        check_recursive(check, agg * val, &rest[1..])
            || check_recursive(check, agg + val, &rest[1..])
            || check_recursive(check, pipe(agg, val), &rest[1..])
    }
}

struct Line {
    to_check: u64,
    vals: Vec<u64>,
}

impl Line {
    fn is_valid(&self) -> bool {
        return check_recursive(self.to_check, 0, self.vals.as_slice());
    }
}

pub fn solve(input: &str) -> u64 {
    let res = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(":").collect();
            let sum: u64 = split[0].parse().unwrap();
            let vals = split[1].to_numbers(" ");
            let count = vals.iter().count() as u64;
            Line {
                to_check: sum,
                vals: vals,
            }
        })
        .filter(|line| line.is_valid())
        .map(|line| line.to_check)
        .sum();

    return res;
}
