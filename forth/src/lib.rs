pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::<Value>::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let token_stream = input
            .split_whitespace()
            .map(|token| token.to_ascii_uppercase());
        for token in token_stream {
            if let Ok(number) = token.parse::<i32>() {
                self.stack.push(number)
            } else {
                let binary = "+ - * / SWAP OVER";
                match token.as_str() {
                    s if binary.split_whitespace().any(|operator| s == operator) => {
                        self.binary_operation(&s)?
                    }
                    "DUP" => self.unary_operation(&token)?,
                    "DROP" => self.unary_operation(&token)?,
                    _ => return Err(Error::UnknownWord),
                }
            }
        }

        Ok(())
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        if let Some(operand) = self.stack.pop() {
            Ok(operand)
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn binary_operation(&mut self, operator: &str) -> Result {
        let (op1, op2) = (self.pop()?, self.pop()?);
        match operator {
            "+" => self.stack.push(op2 + op1),
            "-" => self.stack.push(op2 - op1),
            "*" => self.stack.push(op2 * op1),
            "/" => {
                if op1 == 0 {
                    return Err(Error::DivisionByZero);
                };
                self.stack.push(op2 / op1)
            }
            "SWAP" => {
                self.stack.push(op1);
                self.stack.push(op2);
            }
            "OVER" => {
                self.stack.push(op2);
                self.stack.push(op1);
                self.stack.push(op2);
            }
            _ => return Err(Error::UnknownWord),
        }
        Ok(())
    }

    fn unary_operation(&mut self, operator: &str) -> Result {
        let op = self.pop()?;
        match operator {
            "DUP" => {
                self.stack.push(op);
                self.stack.push(op);
            }
            "DROP" => {}
            _ => return Err(Error::UnknownWord),
        }
        Ok(())
    }
}
