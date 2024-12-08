use crate::AntiNode;
use lib::*;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> u64 {
    let mut antinodes: HashSet<AntiNode> = HashSet::new();

    let mut nodes_per_char: HashMap<char, Vec<AntiNode>> = HashMap::new();

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = map.iter().count();
    let width = map[0].iter().count();

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, char)| {
            if (*char != '.') {
                nodes_per_char
                    .entry(*char)
                    .or_insert(vec![])
                    .push(AntiNode {
                        x: x as i32,
                        y: y as i32,
                    });
            }
        })
    });
    nodes_per_char.iter().for_each(|(char, points)| {
        let pairs = pairs_in_vec(points);
        pairs.iter().for_each(|(left, right)| {
            let diff_x = left.x - right.x;
            let diff_y = left.y - right.y;

            antinodes.insert(AntiNode {
                x: left.x + diff_x,
                y: left.y + diff_y,
            });
            antinodes.insert(AntiNode {
                x: right.x - diff_x,
                y: right.y - diff_y,
            });
        })
    });

    let res = antinodes
        .iter()
        .filter(|it| {
            return it.x >= 0 && it.x < width as i32 && it.y >= 0 && it.y < height as i32;
        })
        .count();

    return res as u64;
}
