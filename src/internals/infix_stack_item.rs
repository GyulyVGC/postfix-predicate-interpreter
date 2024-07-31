use crate::{Operator, Parenthesis};

#[derive(PartialEq)]
pub(crate) enum InfixStackItem {
    Operator(Operator),
    Parenthesis(Parenthesis),
}
