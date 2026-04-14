// ------------------------------------------------------------------------------------------------------------------ //
// MARK: operator
// ------------------------------------------------------------------------------------------------------------------ //

/// Used to represent an operator and its attributes. All methods are const, new operators should not be made
/// outside of this file.
pub struct Operator {
    /// String representation of the operator.
    repr: &'static str,

    /// Bitflags that encode attributes for the operator.
    flags: u32,
}

impl Operator {
    /// Creates a new operator from given string and flags.
    pub(self) const fn new(repr: &'static str, flags: u32) -> Self {
        Self { repr, flags }
    }

    /// Returns whether or not an operator has a flag.
    pub const fn is(&self, flag: u32) -> bool {
        self.flags & flag != 0
    }
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: operators
// ------------------------------------------------------------------------------------------------------------------ //

pub mod ops {
    use super::flags::*;
    use super::*;

    /* math */
    pub const ADD: Operator = Operator::new("+", BINARY & MATH);
    pub const SUB: Operator = Operator::new("-", BINARY & MATH);
    pub const MUL: Operator = Operator::new("*", BINARY & MATH);
    pub const DIV: Operator = Operator::new("/", BINARY & MATH);
    pub const FLR: Operator = Operator::new("//", BINARY & MATH);
    pub const MOD: Operator = Operator::new("%", BINARY & MATH);
    pub const POW: Operator = Operator::new("**", BINARY & MATH);

    /* unary */
    pub const NEG: Operator = Operator::new("-", UNARY & MATH);
    pub const NOT: Operator = Operator::new("!", UNARY & LOGICAL);
    pub const DREF: Operator = Operator::new("*", UNARY & OTHER);

    /* logical & equality */
    pub const AND: Operator = Operator::new("&&", BINARY & LOGICAL);
    pub const OR: Operator = Operator::new("||", BINARY & LOGICAL);
    pub const EQ: Operator = Operator::new("==", BINARY & EQUALITY);
    pub const NEQ: Operator = Operator::new("!=", BINARY & EQUALITY);

    /* comparison */
    pub const LT: Operator = Operator::new("<", BINARY & COMPARISON);
    pub const LTE: Operator = Operator::new("<=", BINARY & COMPARISON);
    pub const GT: Operator = Operator::new(">", BINARY & COMPARISON);
    pub const GTE: Operator = Operator::new(">=", BINARY & COMPARISON);

    /* bitwise */
    pub const BIT_NOT: Operator = Operator::new("~", UNARY & BITWISE);
    pub const BIT_AND: Operator = Operator::new("&", BINARY & BITWISE);
    pub const BIT_OR: Operator = Operator::new("|", BINARY & BITWISE);
    pub const BIT_XOR: Operator = Operator::new("^", BINARY & BITWISE);
    pub const BIT_SHL: Operator = Operator::new("<<", BINARY & BITWISE);
    pub const BIT_SHR: Operator = Operator::new(">>", BINARY & BITWISE);

    /* assignment */
    pub const ADD_ASSIGN: Operator = Operator::new("+=", BINARY & ASSIGNMENT);
    pub const SUB_ASSIGN: Operator = Operator::new("-=", BINARY & ASSIGNMENT);
    pub const MUL_ASSIGN: Operator = Operator::new("*=", BINARY & ASSIGNMENT);
    pub const DIV_ASSIGN: Operator = Operator::new("/=", BINARY & ASSIGNMENT);
    pub const FLR_ASSIGN: Operator = Operator::new("//=", BINARY & ASSIGNMENT);
    pub const MOD_ASSIGN: Operator = Operator::new("%=", BINARY & ASSIGNMENT);
    pub const POW_ASSIGN: Operator = Operator::new("**=", BINARY & ASSIGNMENT);
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: bitflags
// ------------------------------------------------------------------------------------------------------------------ //

pub mod flags {
    /* syntax */
    pub const BINARY: u32 = 1 << 0;
    pub const UNARY: u32 = 1 << 1;

    /* semantics */
    pub const MATH: u32 = 1 << 2;
    pub const LOGICAL: u32 = 1 << 3;
    pub const EQUALITY: u32 = 1 << 4;
    pub const COMPARISON: u32 = 1 << 5;
    pub const BITWISE: u32 = 1 << 6;
    pub const ASSIGNMENT: u32 = 1 << 7;
    pub const OTHER: u32 = 1 << 8;
}
