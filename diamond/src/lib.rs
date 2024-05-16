pub fn get_diamond(c: char) -> Vec<String> {
    let range = ('A'..=c).count();
    let mut diamond = ('A'..=c)
        .enumerate()
        .fold(Vec::<String>::new(), |mut vec, (idx, char)| {
            let padding = row_width.div_ceil(2) - idx;
            let middle = row_width.saturating_sub(padding).saturating_sub(2);
            let mut row = String::new();
            for _i in 0..padding {
                row.push(' ');
            }
            row.push(char);
            if middle > 0 {
                for _i in 0..padding {
                    row.push(' ');
                    row.push(char);
                }
            }
            for _i in 0..padding {
                row.push(' ');
            }
            vec.push(row);
            vec
        });
    if diamond.len() > 1 {
        diamond.append(&mut diamond.clone().into_iter().rev().skip(1).collect())
    }
    diamond
}
