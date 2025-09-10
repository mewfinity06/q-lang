// File attrimutes
#![allow(unused_imports, dead_code)]

// Rust imports

// Third party imports
use anyhow::{Result, bail, format_err};
use plex::lexer;

use crate::log;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    // Keywords
    Const,
    Let,
    Mut,

    Fn,
    Struct,
    Enum,

    // Tokens
    LessThanEqual,    // <=
    GreaterThanEqual, // >=
    DoubleEqual,      // ==
    NotEqual,         // !=
    And,              // &&
    Or,               // ||
    Pipe,             // |>
    Arrow,            // ->
    Assign,           // :=

    OParen,      // (
    CParen,      // )
    OBrace,      // {
    CBrace,      // }
    OBracket,    // [
    CBracket,    // ]
    Semicolon,   // ;
    Colon,       // :
    Comma,       // ,
    Dot,         // .
    Equal,       // =
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Percent,     // %
    Bang,        // !
    LessThan,    // <
    GreaterThan, // >

    // Literals
    Integer(usize),
    Ident(&'a str),
    String(&'a str),

    Whitespace,
    Comment,

    EOF,
}

lexer! {
    fn next_token(text: 'a) -> Result<Token<'a>>;

    r#"[ \t\r\n]+"# => Ok(Token::Whitespace),
    // "C-style" comments (/* .. */) - can't contain "*/"
    r#"/[*](~(.*[*]/.*))[*]/"# => Ok(Token::Comment),
    // "C++-style" comments (// ...)
    r#"//[^\n]*"# => Ok(Token::Comment),

    // Keywords
    "const" => Ok(Token::Const),
    "let" => Ok(Token::Let),
    "mut" => Ok(Token::Mut),
    "fn" => Ok(Token::Fn),
    "struct" => Ok(Token::Struct),
    "enum" => Ok(Token::Enum),

    // Literals
    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => Ok(Token::Ident(text)),
    r#""[^"]*""# => Ok(Token::String(&text[1..text.len()-1])),
    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Ok(Token::Integer(i))
        } else {
            Err(format_err!("integer `{}` is out of range", text))
        }
    }

    // Tokens
    r#"<="# => Ok(Token::LessThanEqual),
    r#">="# => Ok(Token::GreaterThanEqual),
    r#"=="# => Ok(Token::DoubleEqual),
    r#"!="# => Ok(Token::NotEqual),
    r#"\&\&"# => Ok(Token::And),
    r#"\|\|"# => Ok(Token::Or),
    r#"\|>"# => Ok(Token::Pipe),
    r#"->"# => Ok(Token::Arrow),
    r#":="# => Ok(Token::Assign),

    r#"\("# => Ok(Token::OParen),
    r#"\)"# => Ok(Token::CParen),
    r#"{"# => Ok(Token::OBrace),
    r#"}"# => Ok(Token::CBrace),
    r#"\["# => Ok(Token::OBracket),
    r#"\]"# => Ok(Token::CBracket),
    r#";"# => Ok(Token::Semicolon),
    r#":"# => Ok(Token::Colon),
    r#","# => Ok(Token::Comma),
    r#"\."# => Ok(Token::Dot),
    r#"="# => Ok(Token::Equal),
    r#"\+"# => Ok(Token::Plus),
    r#"-"# => Ok(Token::Minus),
    r#"\*"# => Ok(Token::Asterisk),
    r#"/"# => Ok(Token::Slash),
    r#"%"# => Ok(Token::Percent),
    r#"!"# => Ok(Token::Bang),
    r#"<"# => Ok(Token::LessThan),
    r#">"# => Ok(Token::GreaterThan),

    "." => Err(format_err!("unexpected character: `{}`", text)),

}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token<'a>, Span);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (tok, span) = match next_token(self.remaining) {
                Some((Ok(tok), new_remaining)) => {
                    let lo = self.original.len() - self.remaining.len();
                    let hi = self.original.len() - new_remaining.len();
                    self.remaining = new_remaining;
                    (tok, Span { lo, hi })
                }
                Some((Err(e), _)) => {
                    log!(DEBUG, "{}", e);
                    return None;
                }
                None => return None,
            };

            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
