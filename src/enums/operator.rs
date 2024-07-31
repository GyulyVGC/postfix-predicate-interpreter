#[derive(PartialEq, Copy, Clone, Debug)]
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
