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

pub fn print_tree(arena: &Arena<Token>, root: Option<NodeId>) {
    let Some(root) = root else {
        println!("(empty)");
        return;
    };

    println!("{}", arena[root].get());
    let children: Vec<NodeId> = root.children(arena).collect();
    for (i, &child) in children.iter().enumerate() {
        let is_last = i == children.len() - 1;
        print_node(arena, child, "", is_last);
    }
}

fn print_node(arena: &Arena<Token>, node_id: NodeId, prefix: &str, is_last: bool) {
    let marker = if is_last { "└── " } else { "├── " };
    println!("{}{}{}", prefix, marker, arena[node_id].get());

    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
    let children: Vec<NodeId> = node_id.children(arena).collect();
    for (i, &child) in children.iter().enumerate() {
        let is_last_child = i == children.len() - 1;
        print_node(arena, child, &child_prefix, is_last_child);
    }
}
