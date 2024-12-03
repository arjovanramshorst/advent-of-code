use lib::*;

pub fn solve(input: &str) -> i32 {
    let numbers = input.to_numbers_vec_col();

    let res = numbers.iter().fold(1, |acc, vec| {
        let time = vec[0];
        let record = vec[1];
        let matches = (0..time)
            .filter(|hold| (time - hold) * hold > record)
            .count();
        return acc * matches;
    });
    return res as i32;
}
