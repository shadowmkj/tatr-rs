#[derive(Debug)]
pub enum Token {
    TAG(String),
    OR,
    AND,
    NOT,
    INVALID,
}

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            buffer: String::with_capacity(256),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tag_start = false;

        while self.pos < self.src.len() {
            let c = self
                .src
                .chars()
                .nth(self.pos)
                .expect("Error: invalid index"); // Guaranteed to not fail from above condition

            if c.is_whitespace() {
                if self.buffer.len() > 0 {
                    self.pos += 1;
                    let value = match self.buffer.as_str() {
                        "or" => Token::OR,
                        "and" => Token::AND,
                        "not" => Token::NOT,
                        _ => Token::TAG(self.buffer.clone()),
                    };
                    self.buffer.clear();
                    return Some(value);
                }
            }

            if c == '.' && !tag_start {
                tag_start = true;
                self.pos += 1;
                continue;
            }

            self.buffer.push(c);
            self.pos += 1;
        }

        if !self.buffer.is_empty() {
            let value = match self.buffer.as_str() {
                "or" => Token::OR,
                "and" => Token::AND,
                "not" => Token::NOT,
                _ => Token::TAG(self.buffer.clone()),
            };
            self.buffer.clear();
            return Some(value);
        }

        None
    }
}
