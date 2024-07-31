pub trait PredicateEvaluator {
    type Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool;
}
