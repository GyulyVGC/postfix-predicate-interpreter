use crate::enums::operator::Operator;
use crate::Parenthesis;

pub enum InfixToken<Predicate> {
    Parenthesis(Parenthesis),
    Operator(Operator),
    Predicate(Predicate),
}
