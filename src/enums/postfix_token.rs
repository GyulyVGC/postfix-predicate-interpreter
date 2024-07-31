use crate::enums::operator::Operator;

pub enum PostfixToken<Predicate> {
    Operator(Operator),
    Predicate(Predicate),
}
