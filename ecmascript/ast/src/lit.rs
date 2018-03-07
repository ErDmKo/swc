use std::fmt::{self, Display, Formatter};
use swc_common::Span;
use swc_macros::ast_node;

#[ast_node]
pub enum Lit {
    Str(Str),
    Bool(Bool),
    Null(Null),
    Num(Number),
    Regex(Regex),
}

#[ast_node]
pub struct Str {
    pub span: Span,
    pub value: String,
    /// This includes line escape.
    pub has_escape: bool,
}

#[ast_node]
#[derive(Copy)]
pub struct Bool {
    pub span: Span,
    pub value: bool,
}

#[ast_node]
#[derive(Copy)]
pub struct Null {
    pub span: Span,
}

#[ast_node]
pub struct Regex {
    pub span: Span,
    pub exp: String,
    pub flags: RegexFlags,
}

pub type RegexFlags = ::swc_atoms::JsWord;

#[ast_node]
#[derive(Copy)]
pub struct Number {
    pub span: Span,
    pub value: f64,
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.value.is_infinite() {
            if self.value.is_sign_positive() {
                Display::fmt("Infinity", f)
            } else {
                Display::fmt("-Infinity", f)
            }
        } else {
            Display::fmt(&self.value, f)
        }
    }
}
