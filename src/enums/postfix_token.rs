use crate::enums::operator::Operator;

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixToken<Predicate> {
    Operator(Operator),
    Predicate(Predicate),
}
