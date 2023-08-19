use std::option::Option;
use std::process;
use std::process::{Command, exit};


#[derive(Debug)]
enum TokenType {
    Exit,
    IntLit,
    Semi,
}

#[derive(Debug)]
pub(crate) struct Token {
    token_type: TokenType,
    value: Option<String>,
}

pub(crate) struct Tokenizer {
    src: String,
    index: usize,
}

impl Tokenizer {
    pub fn new(src: String) -> Self {
        Self { src, index: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut buf = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                buf.push(self.consume());
                while let Some(ch) = self.peek() {
                    if ch.is_alphanumeric() {
                        buf.push(self.consume());
                    } else {
                        break;
                    }
                }
                if buf == "exit" {
                    tokens.push(Token { token_type: TokenType::Exit, value: None });
                    buf.clear();
                    continue;
                } else {
                    eprintln!("You messed up!");
                    const EXIT_FAILURE: i32 = process;
                    std::process::exit(EXIT_FAILURE);
                }
            } else if ch.is_digit(10) {
                buf.push(self.consume());
                while let Some(ch) = self.peek() {
                    if ch.is_digit(10) {
                        buf.push(self.consume());
                    } else {
                        break;
                    }
                }
                tokens.push(Token { token_type: TokenType::IntLit, value: Some(buf.clone()) });
                buf.clear();
                continue;
            } else if ch == ';' {
                self.consume();
                tokens.push(Token { token_type: TokenType::Semi, value: None });
                continue;
            } else if ch.is_whitespace() {
                self.consume();
                continue;
            } else {
                eprintln!("You messed up!");
                std::process::exit(EXIT_FAILURE);
                //process::exit(process::EXIT_FAILURE);
            }
        }
        self.index = 0;
        tokens
    }

    fn peek(&self) -> Option<char> {
        self.src.chars().nth(self.index)
    }

    fn consume(&mut self) -> char {
        let ch = self.src.chars().nth(self.index).unwrap();
        self.index += 1;
        ch
    }
}
