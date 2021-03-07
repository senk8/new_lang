use super::types::*;
pub type Token = (TokenKind, Pos);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum TokenKind {
    Opr(OperatorKind),
    Key(KeywordKind),
    Type(TypeKind),
    Delim(DelimitorKind),
    Num(usize),
    Id(String),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum OperatorKind {
    /* arithmetic operator */
    Add,
    Sub,
    Star,
    Div,

    /* rerational operator */
    Eq,
    Neq,
    Geq,
    Leq,
    Lt,
    Gt,

    /* others */
    Amp,
    Assign,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum DelimitorKind {
    /* delimitor */
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Semicolon,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum KeywordKind {
    /* statement */
    If,
    Else,
    While,
    For,
    Return,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum TypeKind {
    Int,
    Pointer,
}
