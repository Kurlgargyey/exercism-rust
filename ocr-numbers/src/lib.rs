// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<_> = input.split("\n").collect();
    valid_format(&lines)?;
    Ok(String::new())
}

fn valid_format(lines: &Vec<&str>) -> Result<(), Error> {
    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    for line in lines.iter() {
        if line.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line.len()));
        }
    }
    Ok(())
}

fn zip_lines(lines: &Vec<&str>) -> Vec<String> {
    for block in lines
        .into_iter()
        .map(|line| convert_line_to_triplets(line))
        .collect::<Vec<_>>()
        .chunks(4) {
    }
    Vec::new()
}

fn convert_line_to_triplets(line: &str) -> Vec<String> {
    line.chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
}
