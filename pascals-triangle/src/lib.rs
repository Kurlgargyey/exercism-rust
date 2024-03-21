pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::<Vec<u32>>::with_capacity(row_count as usize);

        for i in 0..row_count {
            rows.push(pascal_row(i));
        }

        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn factorial(n: u32) -> u64 {
    match n {
        n if n <= 1 => 1,
        _ => (n as u64) * factorial(n - 1),
    }
}

fn pascal_number(row: u32, col: u32) -> u32 {
    (factorial(row) / (factorial(col) * factorial(row - col))) as u32
}

fn pascal_row(row: u32) -> Vec<u32> {
    let mut result = Vec::<u32>::new();
    for i in 0..=row {
        result.push(pascal_number(row, i));
    }
    result
}
