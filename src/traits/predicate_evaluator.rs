pub trait PredicateEvaluator {
    type Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool;
}

impl PredicateEvaluator for () {
    type Predicate = bool;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
        *predicate
    }
}
