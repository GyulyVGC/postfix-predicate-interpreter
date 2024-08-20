use crate::enums::operator::Operator;
use crate::Parenthesis;

#[derive(Debug, PartialEq)]
pub enum InfixToken<Predicate> {
    Parenthesis(Parenthesis),
    Operator(Operator),
    Predicate(Predicate),
}
