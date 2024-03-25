pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_maxima = Vec::<((usize, usize), &u64)>::new();
    for (row_idx, row) in input.iter().enumerate() {
        let maximum = row
            .iter()
            .enumerate()
            .fold(None, |acc, (col_idx, height)| {
                if acc == None || *height > acc.1 {
                    Some(((row_idx, col_idx), *height))
                } else {
                    acc
                }
            });
    }
    let mut results = Vec::<(usize, usize)>::new();
    for ((row, col), height) in row_maxima {
        if input.iter().all(|row| !row[col] > *height) {
            results.push((row, col));
        }
    }
    results
}
