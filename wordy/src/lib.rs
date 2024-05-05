use std::{error::Error, str::FromStr};

pub fn answer(command: &str) -> Option<i32> {
    if !command.ends_with('?') {
        return None;
    }

    let command = command.trim_end_matches('?');

    let mut words = command.split_ascii_whitespace().map(|word| {
        word.trim_end_matches("th")
            .trim_end_matches("nd")
            .trim_end_matches("st")
    });

    match (words.next(), words.next()) {
        (Some("What"), Some("is")) => operate(words.collect()),
        _ => None,
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}
impl FromStr for Operation {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plus" => Ok(Operation::Addition),
            "minus" => Ok(Operation::Subtraction),
            "multiplied" => Ok(Operation::Multiplication),
            "divided" => Ok(Operation::Division),
            "raised" => Ok(Operation::Exponentiation),
            _ => Err(format!("{} is not a valid operation", s).into()),
        }
    }
}

fn operate(command: Vec<&str>) -> Option<i32> {
    println!("Operating on {:?}", command);
    let mut next_operation: Option<Operation> = None;
    let mut iter = command.into_iter();
    let mut result: Option<i32> = None;
    loop {
        match iter.next() {
            Some(s) if s.parse::<i32>().is_ok() => {
                result = match &next_operation {
                    Some(operation) if *operation == Operation::Exponentiation => {
                        let op2 = s.parse::<i32>().unwrap();
                        if !(iter.next() == Some("power")) {
                            return None;
                        }
                        Some(run_operation(operation, result.unwrap(), op2))
                    }
                    Some(operation) => Some(run_operation(
                        operation,
                        result.unwrap(),
                        s.parse::<i32>().unwrap(),
                    )),
                    None => s.parse::<i32>().ok(),
                };
                match iter.next() {
                    Some(s) => next_operation = Operation::from_str(s).ok(),
                    _ => return result,
                }
                println!("Next operation is {:?}", next_operation);
                match &next_operation {
                    Some(Operation::Division) | Some(Operation::Multiplication) => {
                        if !(iter.next() == Some("by")) {
                            return None;
                        };
                    }
                    Some(Operation::Exponentiation) => {
                        if !((iter.next(), iter.next()) == (Some("to"), Some("the"))) {
                            return None;
                        }
                    }
                    _ => (),
                }
            }

            s => {
                println!("{:?} is not a digit!", s);
                return None;
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
        Operation::Exponentiation => op1.pow(op2.try_into().unwrap()),
    }
}
