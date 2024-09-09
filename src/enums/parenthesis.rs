use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "value")]
pub enum Parenthesis {
    Open,
    Close,
}
