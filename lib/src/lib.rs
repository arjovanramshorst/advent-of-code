use std::collections::HashSet;
use std::hash::Hash;

pub fn parse_numbers<T: std::str::FromStr>(input: &str, split: &str) -> Vec<T> {
    input
        .split(split)
        .filter_map(|it| it.parse::<T>().ok())
        .collect()
}

pub fn transpose<T: Clone>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let col_count = rows.get(0).map_or(0, |row| row.len());
    (0..col_count)
        .map(|idx| {
            rows.iter()
                .filter_map(|row| row.get(idx).cloned())
                .collect()
        })
        .collect()
}

pub fn pairs_in_vec<T: Clone>(vec: &[T]) -> Vec<(T, T)> {
    let mut pairs = Vec::new();

    for i in 0..vec.len() {
        for j in (i + 1)..vec.len() {
            let pair = (vec[i].clone(), vec[j].clone());
            pairs.push(pair);
        }
    }

    pairs
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub trait NrString<T> {
    fn to_numbers(&self, split: &str) -> Vec<T>;
    fn to_numbers_vec_row(&self, split: &str) -> Vec<Vec<T>>;
    fn to_numbers_vec_col(&self, split: &str) -> Vec<Vec<T>>;
}

impl NrString<u32> for str {
    fn to_numbers(&self, split: &str) -> Vec<u32> {
        parse_numbers(self, split)
    }
    fn to_numbers_vec_row(&self, split: &str) -> Vec<Vec<u32>> {
        self.lines()
            .map(|line| parse_numbers(line, split))
            .collect()
    }
    fn to_numbers_vec_col(&self, split: &str) -> Vec<Vec<u32>> {
        let rows = self.to_numbers_vec_row(split);
        transpose(rows)
    }
}

impl NrString<u64> for str {
    fn to_numbers(&self, split: &str) -> Vec<u64> {
        parse_numbers(self, split)
    }
    fn to_numbers_vec_row(&self, split: &str) -> Vec<Vec<u64>> {
        self.lines()
            .map(|line| parse_numbers(line, split))
            .collect()
    }
    fn to_numbers_vec_col(&self, split: &str) -> Vec<Vec<u64>> {
        let rows = self.to_numbers_vec_row(split);
        transpose(rows)
    }
}

impl NrString<i32> for str {
    fn to_numbers(&self, split: &str) -> Vec<i32> {
        parse_numbers(self, split)
    }

    fn to_numbers_vec_row(&self, split: &str) -> Vec<Vec<i32>> {
        self.lines()
            .map(|line| parse_numbers(line, split))
            .collect()
    }

    fn to_numbers_vec_col(&self, split: &str) -> Vec<Vec<i32>> {
        let rows = self.to_numbers_vec_row(split);
        transpose(rows)
    }
}
