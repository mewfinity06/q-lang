// File attrimutes
#![allow(unused_imports)]

// Rust imports

// Third party imports
use plex::parser;

// Q-lang imports
use crate::ast::*;
use crate::lexer::{Span, Token};

use Token::*;
parser! {
    fn parse_(Token<'static>, Span);

    (a, b) {
        Span {
            row: a.row,
            col: a.col,
            end_row: b.end_row,
            end_col: b.end_col,
        }
    }

    program: Program<'static> {
        stmts[s] => Program { stmts: s }
    }

    stmts: Vec<Expr> {
        => vec![],
        stmts[mut st] expr[e] Semicolon => {
            st.push(e);
            st
        }
    }

    expr: Expr {
        // variable
        Const Ident(name) Assign expr[e] => {
            Expr {
                span: span!(),
                expr: ExprKind::VarDecl { constant: true, mutable: false, name, ty: None, value: Box::new(e) }
            }
        },
        Let Ident(name) Assign expr[e] => {
            Expr {
                span: span!(),
                expr: ExprKind::VarDecl { constant: false, mutable: false, name, ty: None, value: Box::new(e) }
            }
        },
        Let Mut Ident(name) Assign expr[e] => {
            Expr {
                span: span!(),
                expr: ExprKind::VarDecl { constant: false, mutable: true, name, ty: None, value: Box::new(e) }
            }
        },

        // Functions
        Const Ident(name) Colon Fn OParen CParen Arrow ty[t] Equal expr[e] => {
            todo!()
        },
        Const Ident(name) Colon Fn OParen CParen Arrow ty[t] Equal OBrace expr[e] CBrace => {
            todo!()
        },

        // Atom
        atom[a] => a,
    }

    ty: Type {
        Ident(i) => {
            Type::Ident(i)
        }
    }

    atom: Expr {
        Ident(i) => {
            Expr {
                span: span!(),
                expr: ExprKind::Ident(i),
            }
        },
        Integer(i) => {
            Expr {
                span: span!(),
                expr: ExprKind::Integer(i)
            }
        }
        String(s) => {
            Expr {
                span: span!(),
                expr: ExprKind::String(s)
            }
        },
    }
}

pub fn parse<I: Iterator<Item = (Token<'static>, Span)>>(
    i: I,
) -> Result<Program<'static>, (Option<(Token<'static>, Span)>, &'static str)> {
    parse_(i)
}
