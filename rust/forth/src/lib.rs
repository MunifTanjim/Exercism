use std::{collections::HashMap, iter::Peekable};

pub type Value = i32;
pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Stack(Vec<Value>);

impl Stack {
    fn get(&self, mut idx: i32) -> Result<Value> {
        if idx < 0 {
            idx = self.0.len() as i32 + idx;
        }

        if let Some(&v) = self.0.get(idx as usize) {
            Ok(v)
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn take_one(&mut self) -> Result<Value> {
        if let Some(x) = self.0.pop() {
            Ok(x)
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn take_two(&mut self) -> Result<(Value, Value)> {
        if let (Some(y), Some(x)) = (self.0.pop(), self.0.pop()) {
            Ok((x, y))
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn push(&mut self, value: Value) {
        self.0.push(value);
    }
}

#[derive(Debug)]
pub struct Forth {
    stack: Stack,
    pub cmds: HashMap<String, String>,
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
            stack: Stack(vec![]),
            cmds: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack.0
    }

    fn consume(&self, chars: &mut Peekable<std::str::Chars>, char: char) -> bool {
        if let Some(&ch) = chars.peek() {
            if ch == char {
                chars.next();
                return true;
            }
        }
        return false;
    }

    fn consume_whitespace(&self, chars: &mut Peekable<std::str::Chars>) -> Option<char> {
        while let Some(&ch) = chars.peek() {
            if !ch.is_whitespace() {
                return Some(ch);
            }
            chars.next();
        }
        None
    }

    fn consume_word(&self, chars: &mut Peekable<std::str::Chars>) -> String {
        let mut word = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                break;
            }
            word += &ch.to_string();
            chars.next();
        }
        word
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                _ if ch.is_whitespace() => {
                    self.consume_whitespace(&mut chars);
                }
                ch if ch.is_numeric() => {
                    let v: Value = self.consume_word(&mut chars).parse().unwrap();
                    self.stack.push(v);
                }
                '+' | '-' | '*' | '/' => {
                    chars.next();

                    if let Some(cmd) = self.cmds.get(&ch.to_string()) {
                        self.eval(&cmd.to_string())?
                    } else {
                        let (x, y) = self.stack.take_two()?;
                        self.stack.push(match ch {
                            '/' if y == 0 => return Err(Error::DivisionByZero),
                            '+' => x + y,
                            '-' => x - y,
                            '*' => x * y,
                            '/' => x / y,
                            _ => unreachable!(),
                        })
                    }
                }
                ':' => {
                    chars.next();

                    if let Some(ch) = self.consume_whitespace(&mut chars) {
                        if ch.is_numeric() {
                            return Err(Error::InvalidWord);
                        }
                    }

                    let key = self.consume_word(&mut chars).to_lowercase();

                    if let Some(value) = self.cmds.get(&key) {
                        let key_pattern = format!(" {} ", key);
                        let value_pattern = format!(" {} ", value);
                        for v in self.cmds.values_mut().filter(|v| v.contains(&key_pattern)) {
                            *v = v.replace(&key_pattern, &value_pattern);
                        }
                    }

                    let mut value = vec!["".to_string()];

                    while let Some(ch) = self.consume_whitespace(&mut chars) {
                        if ch == ';' {
                            break;
                        }

                        let mut word = self.consume_word(&mut chars).to_lowercase();

                        if key == word {
                            word = self.cmds.get(&word).unwrap().to_string()
                        }

                        value.push(word);
                    }

                    if !self.consume(&mut chars, ';') {
                        return Err(Error::InvalidWord);
                    }

                    chars.next();

                    value.push("".to_string());

                    self.cmds.insert(key, value.join(" "));
                }
                ch if ch.is_alphabetic() => {
                    let word = self.consume_word(&mut chars);

                    match word.to_lowercase().as_str() {
                        cmd if self.cmds.contains_key(cmd) => {
                            self.eval(&self.cmds.get(cmd).unwrap().to_owned())?;
                        }
                        "dup" => {
                            let x = self.stack.get(-1)?;
                            self.stack.push(x);
                        }
                        "drop" => {
                            self.stack.take_one()?;
                        }
                        "swap" => {
                            let (x, y) = self.stack.take_two()?;
                            self.stack.push(y);
                            self.stack.push(x);
                        }
                        "over" => {
                            let x = self.stack.get(-2)?;
                            self.stack.push(x);
                        }
                        _ => return Err(Error::UnknownWord),
                    }
                }
                _ => return Err(Error::InvalidWord),
            }
        }

        Ok(())
    }
}
