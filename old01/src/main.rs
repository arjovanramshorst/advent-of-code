use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution: {}", solve(input));
}

fn solve(input: &str) -> String {
    let regex_str = "one|two|three|four|five|six|seven|eight|nine";
    let re = Regex::new(&format!("{}|[1-9]", regex_str)).unwrap();
    let re_reverse = Regex::new(&format!("{}|[1-9]", reverse(regex_str))).unwrap();

    let res = input
        .lines()
        .map(|l| {
            let first = re.find(l).unwrap().as_str();
            let last = reverse(re_reverse.find(reverse(l).as_str()).unwrap().as_str());
            return as_number(first) * 10 + as_number(last.as_str());
        })
        .reduce(|a, b| a + b);

    format!("{}", res.unwrap())
}

fn reverse(str: &str) -> String {
    str.chars().rev().collect::<String>()
}

fn as_number(str: &str) -> i32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        number => number.parse().unwrap(),
    }
}
