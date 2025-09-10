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

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Const => write!(f, "const"),
            Token::Let => write!(f, "let"),
            Token::Mut => write!(f, "mut"),
            Token::Fn => write!(f, "fn"),
            Token::Struct => write!(f, "struct"),
            Token::Enum => write!(f, "enum"),
            Token::LessThanEqual => write!(f, "<="),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::DoubleEqual => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Pipe => write!(f, "|>"),
            Token::Arrow => write!(f, "->"),
            Token::Assign => write!(f, ":="),
            Token::OParen => write!(f, "("),
            Token::CParen => write!(f, ")"),
            Token::OBrace => write!(f, "{{"),
            Token::CBrace => write!(f, "}}"),
            Token::OBracket => write!(f, "["),
            Token::CBracket => write!(f, "]"),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Equal => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Bang => write!(f, "!"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::Integer(i) => write!(f, "{}", i),
            Token::Ident(i) => write!(f, "{}", i),
            Token::String(s) => write!(f, "{}", s),
            Token::Whitespace => write!(f, "__whitespace__"),
            Token::Comment => write!(f, "__comment__"),
            Token::EOF => write!(f, "__eof__"),
        }
    }
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
    pub row: usize,
    pub col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token<'a>, Span);
    fn next(&mut self) -> Option<Self::Item> {
        // Track line/column for the current position
        let mut row = 1;
        let mut col = 1;
        let consumed = self.original.len() - self.remaining.len();
        for c in self.original.chars().take(consumed) {
            if c == '\n' {
                row += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        loop {
            let (tok, span) = match next_token(self.remaining) {
                Some((Ok(tok), new_remaining)) => {
                    let start_row = row;
                    let start_col = col;
                    let text = &self.remaining[..self.remaining.len() - new_remaining.len()];
                    for c in text.chars() {
                        if c == '\n' {
                            row += 1;
                            col = 1;
                        } else {
                            col += 1;
                        }
                    }
                    self.remaining = new_remaining;
                    (
                        tok,
                        Span {
                            row: start_row,
                            col: start_col,
                            end_row: row,
                            end_col: col,
                        },
                    )
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
