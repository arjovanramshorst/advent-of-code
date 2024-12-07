use std::panic::catch_unwind;

mod ex_1;
mod ex_2;

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1: {}", ex_1::solve(input));
    println!("Solution 2: {}", ex_2::solve(input));
}

fn find_guard(map: &Vec<Vec<char>>) -> Guard {
    let mut guard: Option<Guard> = None;
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| match c {
            '^' | '>' | 'v' | '<' => {
                println!("Guard at {},{}", x, y);
                guard = Some(Guard {
                    location: Point { x, y },
                    direction: Direction::from(*c),
                })
            }
            _ => {}
        })
    });
    return guard.unwrap();
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Finished,
}

impl Direction {
    fn from(c: char) -> Direction {
        match (c) {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => std::panic!(),
        }
    }
    fn step(&self, point: Point) -> Point {
        match (self) {
            Direction::Up => Point {
                x: point.x,
                y: point.y - 1,
            },
            Direction::Down => Point {
                x: point.x,
                y: point.y + 1,
            },
            Direction::Left => Point {
                x: point.x - 1,
                y: point.y,
            },
            Direction::Right => Point {
                x: point.x + 1,
                y: point.y,
            },
            _ => {
                std::panic!()
            }
        }
    }

    fn turn(&self) -> Direction {
        match (self) {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Finished => std::panic!(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Guard {
    location: Point,
    direction: Direction,
}

impl Guard {
    fn walk(&self, map: &Vec<Vec<char>>, obstruction: Option<Point>) -> Guard {
        let res = catch_unwind(|| {
            let new_point = self.direction.step(self.location);
            if (obstruction.is_some() && new_point == obstruction.unwrap()) {
                return '#';
            } else {
                return map[new_point.y][new_point.x];
            }
        });

        match res {
            Err(_) => Guard {
                location: self.location,
                direction: Direction::Finished,
            },
            Ok('#') => Guard {
                location: self.location,
                direction: Direction::turn(&self.direction),
            },
            _ => Guard {
                location: self.direction.step(self.location),
                direction: self.direction,
            },
        }
    }
}
