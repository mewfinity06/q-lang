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
    OParen,           // (
    CParen,           // )
    OBrace,           // {
    CBrace,           // }
    OBracket,         // [
    CBracket,         // ]
    Semicolon,        // ;
    Colon,            // :
    Comma,            // ,
    Dot,              // .
    Arrow,            // ->
    Equal,            // =
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Slash,            // /
    Percent,          // %
    Bang,             // !
    LessThan,         // <
    GreaterThan,      // >
    LessThanEqual,    // <=
    GreaterThanEqual, // >=
    DoubleEqual,      // ==
    NotEqual,         // !=
    And,              // &&
    Or,               // ||
    Pipe,             // |>

    // Literals
    Number(u64),
    Identifier(&'a str),
    String(&'a str),

    EOF,
}
