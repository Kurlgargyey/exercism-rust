use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    thousands: usize,
    hundreds: usize,
    tens: usize,
    ones: usize,
}

impl Roman {
    fn display_thousands(&self) -> String {
        match self.thousands {
            0 => "".to_string(),
            _ => (0..self.thousands).fold(String::with_capacity(self.thousands), |mut res, _| {
                res.push('M');
                res
            }),
        }
    }
    fn display_hundreds(&self) -> String {
        match self.hundreds {
            0 => "".to_string(),
            1..=3 => (0..self.hundreds).fold(String::with_capacity(self.hundreds), |mut res, _| {
                res.push('C');
                res
            }),
            4 => "CD".to_string(),
            5 => "D".to_string(),
            6..=8 => {
                let mut result = "D".to_string();
                result.push_str(&(0..self.hundreds - 5).fold(
                    String::with_capacity(self.hundreds - 5),
                    |mut res, _| {
                        res.push('C');
                        res
                    },
                ));
                result
            }
            _ => "CM".to_string(),
        }
    }
    fn display_tens(&self) -> String {
        match self.tens {
            0 => "".to_string(),
            1..=3 => (0..self.tens).fold(String::with_capacity(self.tens), |mut res, _| {
                res.push('X');
                res
            }),
            4 => "XL".to_string(),
            5 => "L".to_string(),
            6..=8 => {
                let mut result = "L".to_string();
                result.push_str(&(0..self.tens - 5).fold(
                    String::with_capacity(self.tens - 5),
                    |mut res, _| {
                        res.push('X');
                        res
                    },
                ));
                result
            }
            _ => "XC".to_string(),
        }
    }
    fn display_ones(&self) -> String {
        match self.ones {
            0 => "".to_string(),
            1..=3 => (0..self.ones).fold(String::with_capacity(self.ones), |mut res, _| {
                res.push('I');
                res
            }),
            4 => "IV".to_string(),
            5 => "V".to_string(),
            6..=8 => {
                let mut result = "V".to_string();
                result.push_str(&(0..self.ones - 5).fold(
                    String::with_capacity(self.ones - 5),
                    |mut res, _| {
                        res.push('I');
                        res
                    },
                ));
                result
            }
            _ => "IX".to_string(),
        }
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}{}{}{}",
            self.display_thousands(),
            self.display_hundreds(),
            self.display_tens(),
            self.display_ones()
        )
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let thousands = (num / 1000) as usize;
        let remainder = num % 1000;
        let hundreds = (remainder / 100) as usize;
        let remainder = remainder % 100;
        let tens = (remainder / 10) as usize;
        let remainder = remainder % 10;
        let ones = remainder as usize;

        Roman {
            thousands,
            hundreds,
            tens,
            ones,
        }
    }
}
