mod ex_1;
mod ex_2;

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1: {}", ex_1::solve(input));
    println!("Solution 2: {}", ex_2::solve(input));
}

trait NrString {
    fn to_numbers(&self) -> Vec<i32>;
}

impl NrString for String {
    fn to_numbers(&self) -> Vec<i32> {
        return self
            .split_whitespace()
            .map(|it| it.parse())
            .filter_map(|it| if it.is_ok() { Some(it.unwrap()) } else { None })
            .collect();
    }
}
