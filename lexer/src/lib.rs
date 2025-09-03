use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;
use anyhow::bail;

pub mod token;

pub struct Lexer<'a> {
    file_name: &'a str,
    content: Peekable<Chars<'a>>,
    content_bytes: &'a str,
    cur: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file_name: &'a str, content: &'a str) -> Self {
        Self {
            file_name,
            content: content.chars().peekable(),
            content_bytes: content,
            cur: 0,
        }
    }

    fn next_token(&mut self) -> anyhow::Result<Token<'a>> {
        let start = self.cur;
        match self.content.peek() {
            Some(c) if c.is_whitespace() => {
                self.content.next();
                self.cur += 1;
                self.next_token()
            }
            Some(c) if c.is_alphabetic() || *c == '_' => {
                let len = self.read_word()?;
                let word = &self.content_bytes[start..start + len];

                let tk = if word == "const" {
                    Token::Const
                } else if word == "let" {
                    Token::Let
                } else if word == "mut" {
                    Token::Mut
                } else if word == "fn" {
                    Token::Fn
                } else if word == "struct" {
                    Token::Struct
                } else if word == "enum" {
                    Token::Enum
                } else {
                    Token::Identifier(word)
                };

                Ok(tk)
            }
            Some(c) if c.is_ascii_digit() => {
                let len = self.read_number()?;
                let number_str = &self.content_bytes[start..start + len];
                let number = number_str.parse::<u64>()?;
                Ok(Token::Number(number))
            }
            Some('"') => {
                let len = self.read_string()?;
                let string_literal = &self.content_bytes[start..start + len];
                Ok(Token::String(string_literal))
            }
            Some('\'') => {
                let len = self.read_char()?;
                let char_literal = &self.content_bytes[start..start + len];
                Ok(Token::String(char_literal))
            }
            Some('(') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::OParen)
            }
            Some(')') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::CParen)
            }
            Some('{') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::OBrace)
            }
            Some('}') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::CBrace)
            }
            Some('[') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::OBracket)
            }
            Some(']') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::CBracket)
            }
            Some(';') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::Semicolon)
            }
            Some(':') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::Colon)
            }
            Some(',') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::Comma)
            }
            Some('.') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::Dot)
            }
            Some('-') => {
                self.content.next();
                self.cur += 1;
                if let Some('>') = self.content.peek() {
                    self.content.next();
                    self.cur += 1;
                    Ok(Token::Arrow)
                } else {
                    Ok(Token::Minus)
                }
            }
            Some('=') => {
                self.content.next();
                self.cur += 1;
                if let Some('=') = self.content.peek() {
                    self.content.next();
                    self.cur += 1;
                    Ok(Token::DoubleEqual)
                } else {
                    Ok(Token::Equal)
                }
            }
            Some('+') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::Plus)
            }
            Some('*') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::Asterisk)
            }
            Some('/') => {
                self.content.next();
                self.cur += 1;
                if let Some('/') = self.content.peek() {
                    let _ = self.read_comment()?;
                    self.next_token()
                } else {
                    Ok(Token::Slash)
                }
            }
            Some('%') => {
                self.content.next();
                self.cur += 1;
                Ok(Token::Percent)
            }
            Some('!') => {
                self.content.next();
                self.cur += 1;
                if let Some('=') = self.content.peek() {
                    self.content.next();
                    self.cur += 1;
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::Bang)
                }
            }
            Some('<') => {
                self.content.next();
                self.cur += 1;
                if let Some('=') = self.content.peek() {
                    self.content.next();
                    self.cur += 1;
                    Ok(Token::LessThanEqual)
                } else {
                    Ok(Token::LessThan)
                }
            }
            Some('>') => {
                self.content.next();
                self.cur += 1;
                if let Some('=') = self.content.peek() {
                    self.content.next();
                    self.cur += 1;
                    Ok(Token::GreaterThanEqual)
                } else {
                    Ok(Token::GreaterThan)
                }
            }
            Some('&') => {
                self.content.next();
                self.cur += 1;
                if let Some('&') = self.content.peek() {
                    self.content.next();
                    self.cur += 1;
                    Ok(Token::And)
                } else {
                    bail!("Unexpected character '&'");
                }
            }
            Some('|') => {
                self.content.next();
                self.cur += 1;
                if let Some('|') = self.content.peek() {
                    self.content.next();
                    self.cur += 1;
                    Ok(Token::Or)
                } else if let Some('>') = self.content.peek() {
                    self.content.next();
                    self.cur += 1;
                    Ok(Token::Pipe)
                } else {
                    bail!("Unexpected character '|'");
                }
            }
            Some(c) => anyhow::bail!("Unrecognized character: {}", c),
            None => Ok(Token::EOF),
        }
    }

    fn read_word(&mut self) -> anyhow::Result<usize> {
        let start = self.cur;
        while let Some(c) = self.content.peek() {
            if c.is_alphanumeric() || *c == '_' {
                self.content.next();
                self.cur += 1;
            } else {
                break;
            }
        }
        Ok(self.cur - start)
    }

    fn read_number(&mut self) -> anyhow::Result<usize> {
        let start = self.cur;
        while let Some(c) = self.content.peek() {
            if c.is_ascii_digit() {
                self.content.next();
                self.cur += 1;
            } else {
                break;
            }
        }
        Ok(self.cur - start)
    }

    fn read_string(&mut self) -> anyhow::Result<usize> {
        let start = self.cur;
        self.content.next(); // Skip the opening quote
        self.cur += 1;
        while let Some(c) = self.content.peek() {
            if *c == '"' {
                self.content.next(); // Skip the closing quote
                self.cur += 1;
                return Ok(self.cur - start);
            } else {
                self.content.next();
                self.cur += 1;
            }
        }
        bail!("Unterminated string literal");
    }

    fn read_char(&mut self) -> anyhow::Result<usize> {
        let start = self.cur;
        self.content.next(); // Skip the opening quote
        self.cur += 1;
        if let Some(_) = self.content.next() {
            self.cur += 1;
            if let Some(next_c) = self.content.peek() {
                if *next_c == '\'' {
                    self.content.next(); // Skip the closing quote
                    self.cur += 1;
                    return Ok(self.cur - start);
                }
            }
        }
        bail!("Invalid character literal");
    }

    fn read_comment(&mut self) -> anyhow::Result<usize> {
        let start = self.cur;
        while let Some(c) = self.content.peek() {
            if *c == '\n' {
                break;
            } else {
                self.content.next();
                self.cur += 1;
            }
        }
        Ok(self.cur - start)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) => Some(token),
            Err(err) => {
                utils::log!(ERROR, "(Lexer err) {}: {}", self.file_name, err);
                None
            }
        }
    }
}
