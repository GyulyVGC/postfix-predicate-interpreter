pub trait RpnPredicateEvaluator {
    type Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool;
}
