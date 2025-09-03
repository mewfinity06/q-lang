use std::iter::Peekable;

use anyhow::bail;
use lexer::Lexer;
use node::*;

pub mod node;

pub struct Parser<'a> {
    file_name: &'a str,
    lexer: Peekable<Lexer<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(file_name: &'a str, lexer: Lexer<'a>) -> Self {
        Self {
            file_name: file_name,
            lexer: lexer.peekable()
        }
    }

    fn next_node(&mut self) -> anyhow::Result<Node> {
        match self.lexer.peek() {
            Some(t) => bail!("Unhandled token: {:?}", t),
            None => Ok(Node::EOF)
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_node() {
            Ok(n) => Some(n),
            Err(err) =>  {
                utils::log!(ERROR, "(Parser err) {}: {}", self.file_name, err);
                None
            }
        }
    }
}