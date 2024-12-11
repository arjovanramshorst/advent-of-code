use lib::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Entry {
    idx: usize,
    count: usize,
    value: Option<usize>,
}

impl Entry {
    fn score(&self) -> usize {
        if self.value.is_none() {
            0
        } else {
            ((self.idx)..(self.idx + self.count))
                .map(|idx| idx * self.value.unwrap())
                .sum()
        }
    }
}

pub fn solve(input: &str) -> u64 {
    let mut arr_map: Vec<Entry> = vec![];
    let mut idx = 0;
    input.chars().enumerate().for_each(|(i, char)| {
        let count = if (!char.is_digit(10)) {
            0
        } else {
            char.to_digit(10).unwrap() as usize
        };

        let value = if (i % 2 == 0) {
            Some(i.checked_div(2).unwrap())
        } else {
            None
        };
        arr_map.push(Entry { value, count, idx });
        idx += count;
    });

    let mut empty_map: Vec<Entry> = arr_map
        .iter()
        .filter(|it| it.value.is_none())
        .map(|it| it.clone())
        .collect();

    let mut value_map: Vec<&Entry> = arr_map.iter().filter(|it| it.value.is_some()).collect();
    let mut res: Vec<Entry> = vec![];

    for i in (0..value_map.iter().count()).rev() {
        let entry = value_map[i];
        let empty = empty_map
            .iter()
            .enumerate()
            .find(|(idx, empty)| empty.count >= entry.count && empty.idx < entry.idx)
            .clone();

        match (empty) {
            Some((idx, val)) => {
                // add to res
                res.push(Entry {
                    count: entry.count,
                    idx: val.idx,
                    value: entry.value,
                });

                // Update empty map
                empty_map[idx].count -= entry.count;
                empty_map[idx].idx += entry.count;
            }
            None => {
                // Add directly to res
                res.push(entry.clone())
            }
        }
    }

    let checksum = res
        .iter()
        .map(|entry| entry.score())
        .reduce(|agg, n| agg + n)
        .unwrap();

    return checksum as u64;
}
