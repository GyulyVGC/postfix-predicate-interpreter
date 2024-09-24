pub trait PredicateEvaluator {
    type Predicate;
    type Reason;

    fn evaluate_predicate(
        &self,
        predicate: &Self::Predicate,
        reasons: &mut Vec<Self::Reason>,
    ) -> bool;
}

// impl PredicateEvaluator for () {
//     type Predicate = bool;
//
//     fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
//         *predicate
//     }
// }
