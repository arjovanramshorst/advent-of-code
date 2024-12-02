use crate::NrString;

pub fn solve(input: &str) -> usize {
    let res = input
        .lines()
        .filter(|it| {
            let numbers = it.to_string().to_numbers();
            let flat = invert_window(numbers);
            let res = flat.iter().any(|it| {
                let inc = it.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
                let dec = it.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);
                return inc || dec;
            });

            return res;
        })
        .count();

    return res;
}

fn invert_window<T: Copy>(list: Vec<T>) -> Vec<Vec<T>> {
    list.iter()
        .enumerate()
        .map(|(idx, _)| {
            let mut copy = list.clone();
            copy.remove(idx);
            return copy;
        })
        .collect()
}
