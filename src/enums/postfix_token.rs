use serde::{Deserialize, Serialize};
use crate::enums::operator::Operator;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PostfixToken<Predicate> {
    Operator(Operator),
    Predicate(Predicate),
}
