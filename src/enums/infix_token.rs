use crate::enums::operator::Operator;
use crate::Parenthesis;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum InfixToken<Predicate> {
    Parenthesis(Parenthesis),
    Operator(Operator),
    Predicate(Predicate),
}
