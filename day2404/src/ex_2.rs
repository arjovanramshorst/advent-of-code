use crate::Point;

pub fn solve(input: &str) -> i32 {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let x = chars[0].iter().count();
    let y = chars.iter().count();

    let mut count = 0;
    for x_i in 0..x - 2 {
        for y_i in 0..y - 2 {
            count += is_correct(&chars, Point::cross(x_i, y_i));
        }
    }
    return count;
}

fn is_correct(arr: &Vec<Vec<char>>, points: Vec<Point>) -> i32 {
    let str: String = points.iter().map(|point| arr[point.y][point.x]).collect();
    return if (str == "MMASS" || str == "MSAMS" || str == "SMASM" || str == "SSAMM") {
        1
    } else {
        0
    };
}
