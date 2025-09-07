#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Stmt(Stmt),
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Decl(Decl),
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decl {
    Const { name: String, value: Expr },
    Let { name: String, mutable: bool, value: Expr },
    Fn { name: String, params: Vec<String>, body: Vec<Stmt> },
    Struct { name: String, fields: Vec<String> },
    Enum { name: String, variants: Vec<String> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Atom(Atom),
    Addition(Box<Expr>, Box<Expr>), // left + right
    Subtraction(Box<Expr>, Box<Expr>), // left - right
    Multiplication(Box<Expr>, Box<Expr>), // left * right
    Division(Box<Expr>, Box<Expr>), // left / right
    Power(Box<Expr>, Box<Expr>), // left ^ right
}

#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
    Number(u64),
    Identifier(String),
    String(String),
}