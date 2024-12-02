use crate::NrString;

pub fn solve(input: &str) -> usize {
    let res = input
        .lines()
        .filter(|it| {
            let numbers = it.to_string().to_numbers();
            let inc = numbers
                .windows(2)
                .all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
            let dec = numbers
                .windows(2)
                .all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);
            return inc || dec;
        })
        .count();

    return res;
}
