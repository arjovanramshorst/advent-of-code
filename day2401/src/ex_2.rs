use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut left = Vec::new();
    let mut right = Vec::new();
    input.lines().for_each(|it| {
        let vals: Vec<usize> = it
            .split_whitespace()
            .map(|it| it.parse::<usize>().unwrap())
            .collect();
        left.push(vals[0]);
        right.push(vals[1]);
    });
    left.sort();
    let counts = right.iter().fold(HashMap::new(), |mut agg, x| {
        *agg.entry(x).or_insert(0) += 1_i32;
        return agg;
    });

    let mut sum = 0;
    for i in 0..left.iter().count() {
        let key = left[i];
        sum += counts.get(&key).unwrap_or(&0) * key as i32
    }

    return sum as usize;
}
