use serde::{Deserialize, Serialize};
use crate::enums::operator::Operator;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PostfixToken<Predicate> {
    Operator(Operator),
    Predicate(Predicate),
}
