pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond = ('A'..=c).fold(Vec::<String>::new(), |mut vec, char| {
        let row = get_diamond_row(c, char);
        vec.push(row);
        vec
    });
    if diamond.len() > 1 {
        diamond.append(&mut diamond.clone().into_iter().rev().skip(1).collect())
    }
    diamond
}

fn get_diamond_row(final_char: char, c: char) -> String {
    let mut result = String::new();
    for letter in ('A'..=final_char).rev() {
        if letter == c {
            println!("found {}, pushing letter", c);
            result.push(letter)
        } else {
            println!("found {}, pushing whitespace", c);
            result.push(' ')
        }
    }

    result.push_str(&result.chars().rev().skip(1).collect::<String>());

    result
}
