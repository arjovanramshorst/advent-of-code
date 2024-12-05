use lib::NrString;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> i32 {
    let res: Vec<&str> = input.split("\n\n").collect();
    let ordering = res[0]
        .to_numbers_vec_row("|")
        .iter()
        .fold(HashMap::new(), |mut acc, it| {
            acc.entry(it[1]).or_insert(Vec::new()).push(it[0]);
            acc
        });

    let updates = res[1].to_numbers_vec_row(",");
    let filtered: Vec<&Vec<i32>> = updates
        .iter()
        .filter(|update| {
            let mut invalid = HashSet::new();
            return !update.iter().all(|it| {
                if (invalid.contains(it)) {
                    return false;
                } else {
                    let to_invalidate = ordering.get(it);
                    if (to_invalidate.is_some()) {
                        to_invalidate.unwrap().iter().for_each(|illegal| {
                            invalid.insert(illegal);
                        });
                    }
                    return true;
                }
            });
        })
        .collect();

    let sum = filtered
        .iter()
        .map(|it| {
            let mut sorted = it.iter().copied().collect::<Vec<i32>>();
            sorted.sort_by(|a, b| {
                if (ordering.contains_key(a) && ordering[a].contains(b)) {
                    return std::cmp::Ordering::Less;
                } else if (ordering.contains_key(b) && ordering[b].contains(a)) {
                    return std::cmp::Ordering::Greater;
                } else {
                    return std::cmp::Ordering::Equal;
                }
            });
            sorted
        })
        .map(|it| {
            let count = it.iter().count();
            return it[(count - 1) / 2];
        })
        .sum();

    return sum;
}
