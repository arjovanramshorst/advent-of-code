pub trait NrString {
    fn to_numbers(&self, split: &str) -> Vec<i32>;
    fn to_numbers_vec_row(&self, split: &str) -> Vec<Vec<i32>>;
    fn to_numbers_vec_col(&self, split: &str) -> Vec<Vec<i32>>;
}

impl NrString for str {
    fn to_numbers(&self, split: &str) -> Vec<i32> {
        return self
            .split(split)
            .map(|it| it.parse())
            .filter_map(|it| if it.is_ok() { Some(it.unwrap()) } else { None })
            .collect();
    }

    fn to_numbers_vec_row(&self, split: &str) -> Vec<Vec<i32>> {
        return self.lines().map(|line| line.to_numbers(split)).collect();
    }

    fn to_numbers_vec_col(&self, split: &str) -> Vec<Vec<i32>> {
        let rows = self.to_numbers_vec_row(split);
        let col_count = rows[0].iter().count();
        let cols = (0..col_count)
            .map(|idx| rows.iter().map(|row| row[idx]).collect())
            .collect();
        return cols;
    }
}
