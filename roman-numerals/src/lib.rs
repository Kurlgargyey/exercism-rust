use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub struct Roman<'r> {
    val: u32,
    numerals: HashMap<u32, HashMap<&'r str, char>>,
    /*
    thousands_numerals: HashMap<&'r str, char>,
    hundreds_numerals: HashMap<&'r str, char>,
    tens_numerals: HashMap<&'r str, char>,
    ones_numerals: HashMap<&'r str, char>,
    thousands: usize,
    hundreds: usize,
    tens: usize,
    ones: usize,
    */
}

impl Roman<'_> {
    fn display_decimal(&self, decimal: u32) -> String {
        let decimal_val = self.val / decimal % 10;
        let decimal_numerals = &self.numerals[&decimal];
        match decimal_val {
            0 => "".to_string(),
            1..=3 => (0..decimal_val).fold(String::new(), |mut res, _| {
                res.push(decimal_numerals["numeral"]);
                res
            }),
            4 => {
                let mut result = decimal_numerals["numeral"].to_string();
                result.push(decimal_numerals["fiver"]);
                result
            }
            5 => decimal_numerals["fiver"].to_string(),
            6..=8 => {
                (0..decimal_val - 5).fold(decimal_numerals["fiver"].to_string(), |mut res, _| {
                    res.push(decimal_numerals["numeral"]);
                    res
                })
            }
            _ => {
                let mut result = decimal_numerals["numeral"].to_string();
                result.push(self.numerals[&(decimal * 10)]["numeral"]);
                result
            }
        }
    }

    /*
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
    */
}

impl Display for Roman<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}{}{}{}",
            self.display_decimal(1000),
            self.display_decimal(100),
            self.display_decimal(10),
            self.display_decimal(1)
        )
    }
}

impl From<u32> for Roman<'_> {
    fn from(num: u32) -> Self {
        let thousands = (num / 1000) as usize;
        let remainder = num % 1000;
        let hundreds = (remainder / 100) as usize;
        let remainder = remainder % 100;
        let tens = (remainder / 10) as usize;
        let remainder = remainder % 10;
        let ones = remainder as usize;

        let val = num;
        let mut numerals = HashMap::<u32, HashMap<&str, char>>::new();
        let thousands_numerals = HashMap::from([("numeral", 'M')]);
        numerals.insert(1000, thousands_numerals);
        let hundreds_numerals = HashMap::from([("numeral", 'C'), ("fiver", 'D')]);
        numerals.insert(100, hundreds_numerals);
        let tens_numerals = HashMap::from([("numeral", 'X'), ("fiver", 'L')]);
        numerals.insert(10, tens_numerals);
        let ones_numerals = HashMap::from([("numeral", 'I'), ("fiver", 'V')]);
        numerals.insert(1, ones_numerals);

        Roman { val, numerals }
    }
}
