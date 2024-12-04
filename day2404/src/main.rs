extern crate core;

mod ex_1;
mod ex_2;

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1: {}", ex_1::solve(input));
    println!("Solution 2: {}", ex_2::solve(input));
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn hor(x: usize, y: usize) -> Vec<Point> {
        (x..=x + 3)
            .map(|x_i| Point { x: x_i, y: y })
            .collect::<Vec<Point>>()
    }
    fn ver(x: usize, y: usize) -> Vec<Point> {
        (y..=y + 3)
            .map(|y_i| Point { x: x, y: y_i })
            .collect::<Vec<Point>>()
    }
    fn diag(x: usize, y: usize) -> Vec<Point> {
        (0..=3)
            .map(|i| Point { x: x + i, y: y + i })
            .collect::<Vec<Point>>()
    }
    fn diag_2(x: usize, y: usize) -> Vec<Point> {
        (0..=3)
            .map(|i| Point {
                x: x + i,
                y: y + 3 - i,
            })
            .collect::<Vec<Point>>()
    }

    fn cross(x: usize, y: usize) -> Vec<Point> {
        vec![
            Point { x: x, y: y },
            Point { x: x + 2, y: y },
            Point { x: x + 1, y: y + 1 },
            Point { x: x, y: y + 2 },
            Point { x: x + 2, y: y + 2 },
        ]
    }
}
