use crate::enums::rpn_operator::RpnOperator;

pub enum RpnToken<Predicate> {
    Operator(RpnOperator),
    Predicate(Predicate),
}
