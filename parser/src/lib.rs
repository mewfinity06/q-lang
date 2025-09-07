use std::iter::Peekable;

use anyhow::bail;
use lexer::Lexer;
use lexer::token::Token;
use node::*;

pub mod node;

pub struct Parser<'a> {
    file_name: &'a str,
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(file_name: &'a str, lexer: Lexer<'a>) -> Self {
        Self {
            file_name: file_name,
            lexer: lexer.peekable(),
        }
    }

    fn next_node(&mut self) -> anyhow::Result<Node> {
        match self.lexer.peek() {
            None | Some(Token::EOF) => Ok(Node::EOF),
            Some(t) => bail!("Unhandled token: {:?}", t),
        }
    }

    fn parse_decl(&mut self) -> anyhow::Result<Decl> {
        match self.lexer.peek() {
            Some(Token::Const) => {
                self.lexer.next();
                // Expect identifier
                let name = match self.lexer.next() {
                    Some(Token::Identifier(name)) => name.to_string(),
                    other => bail!("Expected identifier after 'const', found {:?}", other),
                };
                // Expect '='
                match self.lexer.next() {
                    Some(Token::Equal) => {}
                    other => bail!("Expected '=' after const name, found {:?}", other),
                }
                // Expect expression (for now, just a number)
                let value = match self.lexer.next() {
                    Some(Token::Number(n)) => Expr::Atom(Atom::Number(n)),
                    other => bail!("Expected number after '=', found {:?}", other),
                };
                // Expect ';'
                match self.lexer.next() {
                    Some(Token::Semicolon) => {}
                    other => bail!("Expected ';' after const declaration, found {:?}", other),
                }
                Ok(Decl::Const { name, value })
            }
            Some(Token::Let) => {
                self.lexer.next();
                // Expect identifier
                let name = match self.lexer.next() {
                    Some(Token::Identifier(name)) => name.to_string(),
                    other => bail!("Expected identifier after 'let', found {:?}", other),
                };
                // Check for 'mut'
                let mutable = match self.lexer.peek() {
                    Some(Token::Mut) => {
                        self.lexer.next();
                        true
                    }
                    _ => false,
                };
                // Expect '='
                match self.lexer.next() {
                    Some(Token::Equal) => {}
                    other => bail!("Expected '=' after let name, found {:?}", other),
                }
                // Expect expression (for now, just a number)
                let value = match self.lexer.next() {
                    Some(Token::Number(n)) => Expr::Atom(Atom::Number(n)),
                    other => bail!("Expected number after '=', found {:?}", other),
                };
                // Expect ';'
                match self.lexer.next() {
                    Some(Token::Semicolon) => {}
                    other => bail!("Expected ';' after let declaration, found {:?}", other),
                }
                Ok(Decl::Let { name, mutable, value })
            }
            Some(other) => bail!("Expected declaration, found {:?}", other),
            None => bail!("Unexpected end of input while parsing declaration"),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_node() {
            Ok(n) => Some(n),
            Err(err) => {
                utils::log!(ERROR, "(Parser err) {}: {}", self.file_name, err);
                None
            }
        }
    }
}
