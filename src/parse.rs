use crate::{
    ast::{
        expr::{Expr, ExprKind},
        tree::ParseTree,
    },
    common::{
        diagnostic::Diag,
        file::Substring,
        handle::Handle,
        token::{TK, Token, TokenKind},
    },
    scan,
};

pub struct Parser<'a> {
    input: &'a str,
    scanner: scan::Scanner<'a>,
    parse_tree: ParseTree,
    diag_list: Vec<Diag>,
    path: &'a str,
    tk0: Token,
    tk1: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, path: &'a str) -> Self {
        let mut scanner = scan::Scanner::new(path, input.as_bytes());
        let mut diag_list = Vec::<Diag>::new();

        // Make sure we start with something valid (even if there are errors)
        let mut tk0 = scanner.next();
        while tk0.is_err() || tk0.as_ref().is_ok_and(|token| token.kind == TK::EOF) {
            // Check for EOF (empty file -- don't parse)
            if tk0.as_ref().is_ok_and(|token| token.kind == TK::EOF) {
                todo!("input is EOF on first token!");
            }

            diag_list.push(tk0.err().unwrap());
            tk0 = scanner.next();
        }

        // Repeat with the peeked token (this time no EOF check)
        let mut tk1 = scanner.next();
        while tk1.is_err() {
            diag_list.push(tk1.err().unwrap());
            tk1 = scanner.next();
        }

        // Now combine into a parser and return
        Self {
            input,
            scanner,
            parse_tree: ParseTree::new(),
            diag_list,
            path,
            tk0: tk0.unwrap(),
            tk1: tk1.unwrap(),
        }
    }
}

impl<'a> Parser<'a> {
    fn token(&mut self) -> Token {
        let mut last_diag: Option<Diag> = None;

        loop {
            match self.scanner.next() {
                Ok(t) => return t,
                Err(e) => {
                    // Check to prevent infinite loop
                    if last_diag.is_some_and(|d| d == e) {
                        panic!("infinite error loop");
                    }

                    // Keep track of diagnostics when they come through
                    last_diag = Some(e.clone());
                    self.diag_list.push(e);
                }
            }
        }
    }

    fn eat(&mut self) {
        self.tk0 = self.tk1;
        self.tk1 = self.token();
    }

    fn expect(&mut self, kind: TokenKind) -> bool {
        if self.tk1.kind == kind {
            self.eat();
            return true;
        }
        false
    }
}

impl<'a> Parser<'a> {
    fn parse_atom(&mut self) -> Result<Handle<Expr>, Diag> {
        let start_pos = self.tk0.pos;

        // Make sure token codes for valid lexeme
        debug_assert!(start_pos.offset + start_pos.len - 1 < self.input.len());
        debug_assert!(
            self.input.is_char_boundary(start_pos.offset)
                && self
                    .input
                    .is_char_boundary(start_pos.offset + start_pos.len)
        );
        let lexeme = &self.input[start_pos.as_range()];

        // Parse based on token kind
        match &self.tk0.kind {
            TK::Integer => match lexeme.parse::<i32>() {
                Ok(val) => Ok(self
                    .parse_tree
                    .push_expr(Expr::new(ExprKind::Integer { val }, start_pos))),
                Err(_) => todo!("emit int parse error"),
            },
            TK::Float => match lexeme.parse::<f32>() {
                Ok(val) => Ok(self
                    .parse_tree
                    .push_expr(Expr::new(ExprKind::Float { val }, start_pos))),
                Err(_) => todo!("emit float parse error"),
            },

            TK::True => Ok(self
                .parse_tree
                .push_expr(Expr::new(ExprKind::Boolean { val: true }, start_pos))),
            TK::False => Ok(self
                .parse_tree
                .push_expr(Expr::new(ExprKind::Boolean { val: false }, start_pos))),
            TK::Null => Ok(self
                .parse_tree
                .push_expr(Expr::new(ExprKind::Null, start_pos))),

            TK::Str => {
                debug_assert!(start_pos.len >= 2, "string literal less than len 2!");
                debug_assert!(start_pos.offset + start_pos.len - 2 < self.input.len());
                debug_assert!(
                    self.input.is_char_boundary(start_pos.offset + 1)
                        && self
                            .input
                            .is_char_boundary(start_pos.offset + start_pos.len - 1)
                );
                let val = Substring::from_span(self.input, start_pos.offset, start_pos.len);
                Ok(self
                    .parse_tree
                    .push_expr(Expr::new(ExprKind::String { val }, start_pos)))
            }
            TK::Symbol => {
                let name = Substring::from_range(self.input, start_pos.as_range());
                Ok(self
                    .parse_tree
                    .push_expr(Expr::new(ExprKind::Symbol { name }, start_pos)))
            }
            _ => todo!(),
        }
    }
}
