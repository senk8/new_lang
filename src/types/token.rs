use super::annotation::*;

pub type Token = Annot<TokenKind>;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum TokenKind {
    Opr(OperatorKind),
    Key(KeywordKind),
    Delim(DelimitorKind),
    Num(usize),
    Id(String),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum OperatorKind {
    /* arithmetic operator */
    Add,
    Sub,
    Mul,
    Div,

    /* rerational operator */
    Eq,
    Neq,
    Geq,
    Leq,
    Lt,
    Gt,

    /* others */
    Assign,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum DelimitorKind {
    /* delimitor */
    Lc,
    Rc,
    Semicolon,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum KeywordKind {
    /* statement */
    If,
    While,
    For,
    Return,
}
