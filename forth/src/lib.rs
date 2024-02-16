use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, String>,
    token_stream: Vec<String>,
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
            words: HashMap::<String, String>::new(),
            token_stream: Vec::<String>::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        self.token_stream = input
            .split_whitespace()
            .map(|token| token.to_ascii_uppercase())
            .rev()
            .collect();

        while let Some(token) = self.token_stream.pop() {
            self.eval_token(&token)?;
        }

        Ok(())
    }

    fn eval_token(&mut self, token: &str) -> Result {
        if let Ok(number) = token.parse::<i32>() {
            self.stack.push(number)
        } else {
            let binary = "+ - * / SWAP OVER";
            match token {
                s if self.words.keys().any(|word| word == s) => {
                    let token_stream = self.words[s].split_whitespace();
                    for token in token_stream {
                        self.token_stream.push(token.to_string());
                    }
                }

                s if binary.split_whitespace().any(|operator| s == operator) => {
                    self.binary_operation(&token)?
                }
                "DUP" => self.unary_operation(&token)?,
                "DROP" => self.unary_operation(&token)?,
                ":" => self.define_word()?,
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }

    fn pop_value(&mut self) -> std::result::Result<Value, Error> {
        if let Some(operand) = self.stack.pop() {
            Ok(operand)
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn binary_operation(&mut self, operator: &str) -> Result {
        let (op1, op2) = (self.pop_value()?, self.pop_value()?);
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
        let op = self.pop_value()?;
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

    fn define_word(&mut self) -> Result {
        let word = self.token_stream.pop().ok_or(Error::InvalidWord)?;
        if let Ok(_number) = word.parse::<Value>() {
            return Err(Error::InvalidWord);
        };
        let mut word_sequence: Vec<String> = vec![];
        loop {
            let current_token = self.token_stream.pop().ok_or(Error::InvalidWord)?;
            match current_token.as_str() {
                ";" => break,
                s if self.words.keys().any(|word| word == &s) => self.words[s]
                    .split_whitespace()
                    .map(|token| token.to_string())
                    .for_each(|token| word_sequence.push(token)),
                _ => word_sequence.push(current_token.to_string()),
            }
        }
        let mut word_sequence: String = word_sequence
            .into_iter()
            .map(|word| word.to_ascii_uppercase())
            .rev()
            .collect::<Vec<String>>()
            .join(" ");

        let mut test_forth = Forth::new();
        let test_sequence = word_sequence
            .split_whitespace()
            .map(|word| word.to_string())
            .rev()
            .collect::<Vec<String>>()
            .join(" ");
        println!("Testing sequence {}!", test_sequence);
        if test_forth.eval(&test_sequence).is_ok() && test_forth.stack.is_empty() {
            word_sequence = String::new();
        }

        println!("Inserting sequence {} for word {}!", word_sequence, word);
        self.words.insert(word, word_sequence);
        Ok(())
    }
}
