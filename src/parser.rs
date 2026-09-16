use std::iter::Peekable;

use indextree::{Arena, NodeId};

use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    arena: Arena<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
            arena: Arena::new(),
        }
    }

    fn parse_or(&mut self) -> Option<NodeId> {
        let mut left = self.parse_and()?;
        while let Some(Token::OR) = self.lexer.peek() {
            let token = self.lexer.next().expect("Cannot fail as we peeked");
            let right = self.parse_and()?;
            let new = self.arena.new_node(token);
            new.append(left, &mut self.arena);
            new.append(right, &mut self.arena);
            left = new;
        }
        Some(left)
    }

    fn parse_and(&mut self) -> Option<NodeId> {
        let mut left = self.parse_not()?;
        while let Some(Token::AND) = self.lexer.peek() {
            let token = self.lexer.next().expect("Cannot fail as we peeked");
            let right = self.parse_not()?;
            let new = self.arena.new_node(token);
            new.append(left, &mut self.arena);
            new.append(right, &mut self.arena);
            left = new;
        }
        Some(left)
    }

    fn parse_not(&mut self) -> Option<NodeId> {
        if let Some(Token::NOT) = self.lexer.peek() {
            let token = self.lexer.next().expect("Token exists since we peeked");
            let child = self.parse_not()?;
            let parent = self.arena.new_node(token);
            parent.append(child, &mut self.arena);
            return Some(parent);
        }
        self.parse_tag()
    }

    fn parse_tag(&mut self) -> Option<NodeId> {
        if let Some(Token::TAG(_)) = self.lexer.peek() {
            let token = self.lexer.next().expect("Token exists since we peeked");
            Some(self.arena.new_node(token))
        } else {
            None
        }
    }

    pub fn parse(mut self) -> (Arena<Token>, Option<NodeId>) {
        let root = self.parse_or();
        (self.arena, root)
    }
}
