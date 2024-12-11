use lib::*;

pub fn solve(input: &str) -> u64 {
    let mut space: Vec<usize> = vec![];
    let arr: Vec<usize> = input
        .chars()
        .enumerate()
        .flat_map(|(i, char)| {
            let count: usize = if (!char.is_digit(10)) {
                0
            } else if (i % 2 == 0) {
                char.to_digit(10).unwrap() as usize
            } else {
                &space.push(char.to_digit(10).unwrap() as usize);
                0
            };
            return (0..count).map(move |_| i.checked_div(2).unwrap());
        })
        .collect();

    let mut finger_left: usize = 0;
    let mut finger_right = arr.iter().count() - 1;
    let mut finger_space: usize = 0;

    let mut res: Vec<usize> = vec![];
    let mut last_id_left: usize = 0;
    while (finger_left <= finger_right) {
        if (arr[finger_left] == last_id_left) {
            finger_left += 1;
            // Push from the left:
            res.push(last_id_left)
        } else {
            (0..space[finger_space]).for_each(|_| {
                res.push(arr[finger_right]);
                finger_right -= 1;
            });
            finger_space += 1;
            last_id_left = last_id_left + 1;
        }
    }

    let checksum = res
        .iter()
        .enumerate()
        .map(|(id, idx)| id * idx)
        .reduce(|agg, n| agg + n)
        .unwrap();

    return checksum as u64;
}
