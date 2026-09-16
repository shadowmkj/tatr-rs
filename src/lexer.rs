use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
    TAG(String),
    OR,
    AND,
    NOT,
    INVALID,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::TAG(tag) => write!(f, "TAG({tag})"),
            Token::OR => write!(f, "OR"),
            Token::AND => write!(f, "AND"),
            Token::NOT => write!(f, "NOT"),
            Token::INVALID => write!(f, "INVALID"),
        }
    }
}

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
            buffer: String::with_capacity(256),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&c) = self.src.peek() {
            if c.is_whitespace() {
                self.src.next();
            } else {
                break;
            }
        }

        self.src.peek()?; // Return NONE if EOF (Empty string provided)

        self.buffer.clear();
        while let Some(&c) = self.src.peek() {
            if c.is_whitespace() {
                break;
            }
            self.buffer.push(c);
            self.src.next();
        }

        let word = self.buffer.strip_prefix(':').unwrap_or(&self.buffer);
        let token = match word {
            "and" => Token::AND,
            "or" => Token::OR,
            "not" => Token::NOT,
            _ => Token::TAG(word.to_string()),
        };

        Some(token)
    }
}
