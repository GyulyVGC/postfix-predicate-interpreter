use crate::enums::operator::Operator;

#[derive(Debug, PartialEq)]
pub enum PostfixToken<Predicate> {
    Operator(Operator),
    Predicate(Predicate),
}
