use crate::common::file::Position;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    EOF,

    /* Grouping operators
     */
    LPar,
    RPar,
    LBrac,
    RBrac,
    LCurl,
    RCurl,

    /* Arithmetic operators
     */
    Plus,
    PlusPlus,
    PlusEq,
    Min,
    MinMin,
    MinEq,
    Star,
    StarStar,
    StarEq,
    StarStarEq,
    Slash,
    SlashSlash,
    SlashEq,
    SlashSlashEq,
    Mod,
    ModEq,

    /* Misc operators
     */
    Dot,
    Colon,
    Semicolon,

    /* Comparison operators
     */
    Bang,
    BangEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Eq,
    EqEq,

    /* Logical and bitwise operators
     */
    Bar,
    BarBar,
    Ampsand,
    AmpsandAmpsand,

    /* Literals
     */
    Symbol,
    Integer,
    Float,
    Str,
    True,
    False,
    Null,

    /* Keywords
     */
    Mut,
    Fn,
    For,
    In,
    While,
    If,
    Else,
    Defer,
}

/// A convenience alias for `TokenType` available to the frontend.
pub type TK = TokenKind;

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
}

impl Token {
    pub const fn new(typ: TokenKind, pos: Position) -> Self {
        Self { kind: typ, pos }
    }
}
