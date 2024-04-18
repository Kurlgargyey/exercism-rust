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

    let lines: Vec<String> = lines
        .into_iter()
        .map(|line| {
            line.into_iter().fold(String::new(), |mut line_string, ocr_string| {
                line_string.push(ocr_string.0);
                line_string
            })
        })
        .collect();

    Ok(lines.join(","))
}

struct OCRString(char);

impl OCRString {
    fn new(input: &str) -> Self {
        OCRString(match input {
            " _ | ||_|   " => '0',
            "     |  |   " => '1',
            " _  _||_    " => '2',
            " _  _| _|   " => '3',
            "   |_|  |   " => '4',
            " _ |_  _|   " => '5',
            " _ |_ |_|   " => '6',
            " _   |  |   " => '7',
            " _ |_||_|   " => '8',
            " _ |_| _|   " => '9',
            _ => '?',
        })
    }
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

fn zip_lines(lines: &Vec<&str>) -> Vec<Vec<OCRString>> {
    lines
        .into_iter()
        .map(|line| into_triplets(line))
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|block| zip_into_vec(block))
        .collect::<Vec<_>>()
}

fn zip_into_vec(block: &[Vec<String>]) -> Vec<OCRString> {
    block[0]
        .iter()
        .zip(block[1].iter())
        .zip(block[2].iter().zip(block[3].iter()))
        .fold(Vec::new(), |mut result_vec, quadruplet| {
            result_vec.push(to_ocrstring(quadruplet));
            result_vec
        })
}

fn to_ocrstring(quadruplet: ((&String, &String), (&String, &String))) -> OCRString {
    let ((line1, line2), (line3, line4)) = quadruplet;
    let mut result = String::new();
    result.push_str(line1);
    result.push_str(line2);
    result.push_str(line3);
    result.push_str(line4);
    OCRString::new(&result)
}

fn into_triplets(line: &str) -> Vec<String> {
    line.chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
}
