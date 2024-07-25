pub trait RpnEvaluator {
    type Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool;
}
