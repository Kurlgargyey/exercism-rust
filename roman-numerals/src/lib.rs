use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub struct Roman<'r> {
    val: u32,
    numerals: HashMap<u32, HashMap<&'r str, char>>,
}

impl Roman<'_> {
    fn display_decimal(&self, decimal: u32) -> String {
        let decimal_val = self.val / decimal % 10;
        let decimal_numerals = &self.numerals[&decimal];
        match decimal_val {
            0 => "".to_string(),
            1..=3 => (0..decimal_val)
                .map(|_| decimal_numerals["numeral"])
                .collect(),
            4 => [decimal_numerals["numeral"], decimal_numerals["fiver"]]
                .into_iter()
                .collect(),
            5 => decimal_numerals["fiver"].to_string(),
            6..=8 => [decimal_numerals["fiver"]]
                .into_iter()
                .chain((0..decimal_val - 5).map(|_| decimal_numerals["numeral"]))
                .collect(),

            _ => [
                decimal_numerals["numeral"],
                self.numerals[&(decimal * 10)]["numeral"],
            ]
            .into_iter()
            .collect(),
        }
    }
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
        let val = num;
        let mut numerals = HashMap::<u32, HashMap<&str, char>>::new();
        numerals.insert(1000, HashMap::from([("numeral", 'M')]));
        numerals.insert(100, HashMap::from([("numeral", 'C'), ("fiver", 'D')]));
        numerals.insert(10, HashMap::from([("numeral", 'X'), ("fiver", 'L')]));
        numerals.insert(1, HashMap::from([("numeral", 'I'), ("fiver", 'V')]));

        Roman { val, numerals }
    }
}
