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
    let lines = zip_lines(&lines);
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

fn zip_lines(lines: &Vec<&str>) -> Vec<Vec<String>> {
    lines
        .into_iter()
        .map(|line| convert_line_to_triplets(line))
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|block| zip_quadruplet_to_vec(block))
        .collect::<Vec<_>>()
}

fn zip_quadruplet_to_vec(block: &[Vec<String>]) -> Vec<String> {
    block[0]
        .iter()
        .zip(block[1].iter())
        .zip(block[2].iter().zip(block[3].iter()))
        .fold(Vec::new(), |mut result_vec, ((line1, line2), (line3, line4))| {
            let mut result = String::new();
            result.push_str(line1);
            result.push_str(line2);
            result.push_str(line3);
            result.push_str(line4);
            result_vec.push(result);
            result_vec
        })
}

fn convert_line_to_triplets(line: &str) -> Vec<String> {
    line.chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
}
