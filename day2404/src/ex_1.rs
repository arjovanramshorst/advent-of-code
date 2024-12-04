use crate::Point;

pub fn solve(input: &str) -> i32 {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let x = chars[0].iter().count();
    let y = chars.iter().count();

    let mut count = 0;
    for x_i in 0..x - 3 {
        // horizontal:
        for y_i in 0..y {
            count += is_correct(&chars, Point::hor(x_i, y_i));
        }
    }
    for x_i in 0..x {
        // vertical:
        for y_i in 0..y - 3 {
            count += is_correct(&chars, Point::ver(x_i, y_i));
        }
    }
    for x_i in 0..x - 3 {
        // Diagonal:
        for y_i in 0..y - 3 {
            count += is_correct(&chars, Point::diag(x_i, y_i));
            count += is_correct(&chars, Point::diag_2(x_i, y_i));
        }
    }

    return count;
}

fn is_correct(arr: &Vec<Vec<char>>, points: Vec<Point>) -> i32 {
    let str: String = points.iter().map(|point| arr[point.y][point.x]).collect();
    return if (str == "XMAS" || str == "SAMX") {
        1
    } else {
        0
    };
}
