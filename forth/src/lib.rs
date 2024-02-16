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
        for token in input.split_whitespace() {
            if let Ok(number) = token.parse::<i32>() {
                self.stack.push(number)
            } else {
                match token {
                    "+" => {
                        if let (Some(op1), Some(op2)) = (self.stack.pop(), self.stack.pop()) {
                            self.stack.push(op1 + op2);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    _ => return Err(Error::UnknownWord),
                }
            }
        }

        Ok(())
    }
}
