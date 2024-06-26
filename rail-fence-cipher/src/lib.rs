pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails as usize)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut lines = Vec::<Vec<char>>::with_capacity(self.0);
        for _ in 0..self.0 {
            lines.push(Vec::<char>::with_capacity(text.len() / 2))
        }

        let mut dir_down = false;
        let mut row: usize = 0;

        for char in text.chars() {
            if row == 0 || row == self.0 - 1 {
                dir_down = !dir_down
            }
            lines[row].push(char);

            if dir_down {
                row += 1
            } else {
                row -= 1
            }
        }
        lines
            .into_iter()
            .flatten()
            .filter(|c| !c.is_whitespace())
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut lines = Vec::<Vec<char>>::with_capacity(self.0);
        for _ in 0..self.0 {
            lines.push(Vec::<char>::with_capacity(cipher.len()))
        }
        for _ in 0..=cipher.len() {
            for i in 0..self.0 {
                lines[i].push(' ')
            }
        }

        let mut dir_down = false;
        let (mut row, mut col): (usize, usize) = (0, 0);
        for _ in 0..=cipher.len() {
            if row == 0 || row == self.0 - 1 {
                dir_down = !dir_down
            }

            lines[row][col] = '*';
            col += 1;

            if dir_down {
                row += 1
            } else {
                row -= 1
            }
        }
        let mut chars = cipher.chars();
        for row in 0..self.0 {
            for col in 0..cipher.len() {
                if lines[row][col] == '*' {
                    lines[row][col] = chars.next().unwrap_or('*')
                }
            }
        }

        let mut result = String::new();
        for i in 0..cipher.len() {
            for line in &lines {
                let c = *line.into_iter().nth(i).unwrap();
                if !c.is_whitespace() && !(c == '*') {
                    result.push(c);
                }
            }
        }
        result
    }
}
