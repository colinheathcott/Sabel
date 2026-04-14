use crate::{ast::expr::Expr, common::handle::Handle};

pub struct ParseTree {
    expr_list: Vec<Expr>,
}

impl ParseTree {
    pub fn new() -> Self {
        Self { expr_list: vec![] }
    }

    pub fn push_expr(&mut self, expr: Expr) -> Handle<Expr> {
        let index = self.expr_list.len();
        self.expr_list.push(expr);
        Handle::new(index)
    }

    pub fn get_expr(&mut self, handle: Handle<Expr>) -> &Expr {
        match self.expr_list.get(handle.index()) {
            Some(expr) => expr,
            None => panic!("Handle<Expr> out of bounds!"),
        }
    }
}
