use crate::*;
use std::collections::HashSet;
use std::panic;
use std::panic::catch_unwind;

pub fn solve(input: &str) -> u64 {
    panic::set_hook(Box::new(|_info| {
        // Skip panic output, #hackynessalert
    }));

    let mut visited: HashSet<Guard> = HashSet::new();
    let mut obstructions: HashSet<Point> = HashSet::new();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut guard = find_guard(&map);
    let init_location = guard.location.clone();

    while guard.direction != Direction::Finished {
        let mut temp_guard = guard.clone();
        let mut temp_visited: HashSet<Guard> = HashSet::new();
        let obstruction_option = catch_unwind(|| {
            let option = temp_guard.direction.step(temp_guard.location);
            // Should panic if step is outside
            let char = map[option.y][option.x];
            if char != '#' && option != init_location {
                return option;
            } else {
                panic!()
            }
        });

        if obstruction_option.is_ok() {
            let obstruction = obstruction_option.unwrap();
            while temp_guard.direction != Direction::Finished {
                if visited.contains(&temp_guard) || temp_visited.contains(&temp_guard) {
                    obstructions.insert(obstruction);
                    break;
                }

                temp_visited.insert(temp_guard.clone());
                temp_guard = temp_guard.walk(&map, Some(obstruction));
            }
        }

        visited.insert(guard.clone());
        // Walk with this guard until either finished or a visited square is found
        guard = guard.walk(&map, None);
    }

    return obstructions.iter().count() as u64;
}
