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
    right.sort();

    let mut sum = 0;
    for i in 0..left.iter().count() {
        sum += left[i].abs_diff(right[i]);
    }

    return sum;
}
