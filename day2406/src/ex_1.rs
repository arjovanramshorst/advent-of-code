use crate::*;
use std::collections::HashSet;
use std::panic;

pub fn solve(input: &str) -> u64 {
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
    let mut visited: HashSet<Point> = HashSet::new();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut guard = find_guard(&map);

    while guard.direction != Direction::Finished {
        visited.insert(guard.location.clone());
        guard = guard.walk(&map, None);
    }

    return visited.iter().count() as u64;
}
