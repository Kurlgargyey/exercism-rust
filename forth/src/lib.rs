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
                        let operands = self.pop_operands(2)?;
                        self.stack.push(operands[0] + operands[1]);
                    }
                    "-" => {
                        let operands = self.pop_operands(2)?;
                        self.stack.push(operands[1] - operands[0]);
                    }
                    _ => return Err(Error::UnknownWord),
                }
            }
        }

        Ok(())
    }

    fn pop_operands(&mut self, amount: usize) -> std::result::Result<Vec<Value>, Error> {
        let mut i = 0;
        let mut operands = vec![];
        while i < amount {
            if let Some(operand) = self.stack.pop() {
                operands.push(operand);
            } else {
                return Err(Error::StackUnderflow);
            }
            i += 1;
        }
        Ok(operands)
    }
}
