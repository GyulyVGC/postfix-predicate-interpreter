use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "value")]
pub enum Operator {
    And,
    Or,
}

impl Operator {
    pub(crate) fn precedence(self) -> u8 {
        match self {
            Operator::And => 2,
            Operator::Or => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operators_precedence() {
        assert!(Operator::And.precedence() > Operator::Or.precedence());
    }
}
