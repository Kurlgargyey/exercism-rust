pub fn get_diamond(c: char) -> Vec<String> {
    let row_width = ('A'..=c).count() * 2 - 1;
    let middle_idx = row_width / 2;
    ('A'..=c)
        .enumerate()
        .fold(Vec::<String>::new(), |mut vec, (idx, char)| {
            let mut row = String::with_capacity(row_width);
            let first_idx = middle_idx - idx;
            let second_idx = middle_idx + idx;
            row.replace_range(first_idx..first_idx + 1, char.to_string().as_str());
            row.replace_range(second_idx..second_idx + 1, char.to_string().as_str());
            vec.push(row);
            vec
        })
}
