// File attrimutes
#![allow(unused_imports, dead_code)]

// Rust imports

// Third party imports
use anyhow::{Result, bail};

// Q-lang imports
use crate::lexer::{Span, Token};

#[derive(Debug)]
pub struct Program<'a> {
    pub stmts: Vec<Expr<'a>>,
}

#[derive(Debug)]
pub struct Expr<'a> {
    pub span: Span,
    pub expr: ExprKind<'a>,
}

#[derive(Debug)]
pub enum ExprKind<'a> {
    Ident(&'a str),
    Integer(usize),
    String(&'a str),
    VarDecl {
        constant: bool,
        mutable: bool,
        name: &'a str,
        ty: Option<Type<'a>>,
        value: Box<Expr<'a>>,
    },
    FnDecl {
        name: &'a str,
        params: Vec<Param<'a>>,
        ret_ty: Type<'a>,
        body: Vec<Expr<'a>>,
    },
    FnCall {
        name: &'a str,
        args: Vec<Expr<'a>>,
    },
    MacroCall {
        name: &'a str,
        args: Vec<Expr<'a>>,
    },
}

#[derive(Debug)]
pub struct Param<'a> {
    pub name: &'a str,
    pub value: Type<'a>,
}


#[derive(Debug)]
pub enum Type<'a> {
    Ident(&'a str),
}
