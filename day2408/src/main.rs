mod ex_1;
mod ex_2;

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1: {}", ex_1::solve(input));
    println!("Solution 2: {}", ex_2::solve(input));
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct AntiNode {
    x: i32,
    y: i32,
}
