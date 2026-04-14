use crate::common::{
    file::{Position, Substring},
    handle::Handle,
    operator::Operator,
};

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: expressions
// ------------------------------------------------------------------------------------------------------------------ //

pub struct ArgExpr {
    pub name: Option<Substring>,
    pub val: Handle<Expr>,
    pub pos: u8,
}

pub struct BinaryExpr {
    pub lhs: Handle<Expr>,
    pub rhs: Handle<Expr>,
    pub op: &'static Operator,
}

pub struct UnaryExpr {
    pub operand: Handle<Expr>,
    pub op: &'static Operator,
}

pub struct CallExpr {
    pub callee: Handle<Expr>,
    pub args: Vec<ArgExpr>,
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: expr kind
// ------------------------------------------------------------------------------------------------------------------ //

pub enum ExprKind {
    Null,
    Integer { val: i32 },
    Float { val: f32 },
    Boolean { val: bool },
    String { val: Substring },
    Symbol { name: Substring },

    Call(CallExpr),
    Prefix(UnaryExpr),
    Postfix(UnaryExpr),
    Arithmetic(BinaryExpr),
    Comparison(BinaryExpr),
    Equality(BinaryExpr),
    Logical(BinaryExpr),
    Assignment(BinaryExpr),
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: expr
// ------------------------------------------------------------------------------------------------------------------ //

pub struct Expr {
    pub kind: ExprKind,
    pub pos: Position,
}

impl Expr {
    pub fn new(kind: ExprKind, pos: Position) -> Self {
        Self { kind, pos }
    }
}
