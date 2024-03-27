pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_maxima = Vec::<((usize, usize), &u64)>::new();
    for (row_idx, row) in input.iter().enumerate() {
        let maximum = row
            .iter()
            .enumerate()
            .fold(None, |acc: Option<((usize, usize), &u64)>, (col_idx, height)| {
                if acc == None || height > acc.unwrap().1 {
                    Some(((row_idx, col_idx), height))
                } else {
                    acc
                }
            });

        if let Some(row_maximum) = maximum {
            let tallest_tree = row_maximum.1;
            for (col_idx, height) in row.iter().enumerate() {
                if height == tallest_tree {
                    row_maxima.push(((row_idx, col_idx), height));
                }
            }
        }
    }
    let mut results = Vec::<(usize, usize)>::new();
    for ((row, col), height) in row_maxima {
        println!("checking whether {height} is the lowest height in col {col} of vec {input:?}");
        if input.iter().all(|row| row[col] >= *height) {
            results.push((row, col));
        }
    }
    results
}
