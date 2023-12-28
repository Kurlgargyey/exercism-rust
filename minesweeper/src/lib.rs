pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mines: Vec<Vec<usize>> = gather_mines(minefield);
    return annotate_field(minefield, &mines);
}

fn gather_mines(minefield: &[&str]) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = vec![];
    let mut position: Vec<usize> = vec![0, 0];
    for row in minefield {
        for char in row.chars() {
            match char {
                '*' => result.push(position.clone()),
                _ => (),
            }
            position[1] += 1;
        }
        position[0] += 1;
        position[1] = 0;
    }
    result
}

fn annotate_field(minefield: &[&str], mines: &Vec<Vec<usize>>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut position: Vec<usize> = vec![0, 0];
    for row in minefield {
        result.push(annotate_row(row, &mut position, &mines));
        position[0] += 1;
        position[1] = 0;
    }
    result
}

fn annotate_row(row: &str, position: &mut Vec<usize>, mines: &Vec<Vec<usize>>) -> String {
    let mut row_string = String::new();
    for char in row.chars() {
        match char {
            '*' => row_string.push('*'),
            _ => {
                row_string.push_str(&annotate_mine_count(&position, &mines));
            }
        }
        position[1] += 1;
    }
    row_string
}

fn annotate_mine_count(position: &Vec<usize>, mines: &Vec<Vec<usize>>) -> String {
    let count = mines
        .iter()
        .filter(|mine| mine[0].abs_diff(position[0]) < 2 && mine[1].abs_diff(position[1]) < 2)
        .count();

    if count == 0 {
        " ".to_string()
    } else {
        count.to_string()
    }
}
