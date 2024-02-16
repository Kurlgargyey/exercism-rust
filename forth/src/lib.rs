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
            }
        }

        Ok(())
    }
}
