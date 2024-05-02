use std::{error::Error, str::FromStr};

pub fn answer(command: &str) -> Option<i32> {
    if !command.ends_with('?') {
        return None;
    }

    let command = command.trim_end_matches('?');

    let mut words = command.split_ascii_whitespace();

    match (words.next(), words.next()) {
        (Some("What"), Some("is")) => operate(words.collect()),
        _ => None,
    }
}

enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    //Exponentiation,
}
impl FromStr for Operation {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plus" => Ok(Operation::Addition),
            "minus" => Ok(Operation::Subtraction),
            "multiplied by" => Ok(Operation::Multiplication),
            "divided by" => Ok(Operation::Division),
            _ => Err(format!("{} is not a valid operation", s).into()),
        }
    }
}

fn operate(command: Vec<&str>) -> Option<i32> {
    let mut operation: Option<Operation> = None;
    let mut iter = command.into_iter();
    let mut result: Option<i32> = None;
    loop {
        match iter.next() {
            Some(s) if s.parse::<i32>().is_ok() => {
                result = match &operation {
                    Some(operation) => Some(run_operation(
                        operation,
                        result.unwrap(),
                        s.parse::<i32>().unwrap(),
                    )),
                    None => s.parse::<i32>().ok(),
                };
                match iter.next() {
                    Some(s) => operation = Operation::from_str(s).ok(),
                    _ => return result,
                }
            }

            s => {
                println!("{:?} is not a digit!", s);
                return result;
            }
        }
    }
}

fn run_operation(operation: &Operation, op1: i32, op2: i32) -> i32 {
    match operation {
        Operation::Addition => op1 + op2,
        Operation::Subtraction => op1 - op2,
        Operation::Multiplication => op1 * op2,
        Operation::Division => op1 / op2,
    }
}
