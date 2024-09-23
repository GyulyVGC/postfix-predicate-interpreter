use crate::enums::operator::Operator;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PostfixToken<Predicate> {
    Operator(Operator),
    Predicate(Predicate),
}
