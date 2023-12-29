
pub fn column_iterator<'a, T: Clone>(matrix: &'a [Vec<T>]) -> impl Iterator<Item = Vec<T>> + 'a {
    let num_columns =
         matrix.iter()
            .next()
            .map_or(0, |row| row.len());

    (0..num_columns).map(move |col_index| {
        matrix.iter().filter_map(move |row| row.get(col_index).cloned()).collect()
    })
}

pub fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return matrix.clone();
    }

    let num_rows = matrix.len();
    let num_columns = matrix[0].len();

    (0..num_columns)
        .map(|col_index| {
            (0..num_rows)
                .map(|row_index| matrix[row_index][col_index].clone())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn column_iter_test() {
        let array: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = column_iterator(&array).collect_vec();
        dbg!(result);
    }
}