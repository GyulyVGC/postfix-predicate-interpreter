pub trait PredicateEvaluator {
    type Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate, reasons: &mut Vec<String>) -> bool;
}

// impl PredicateEvaluator for () {
//     type Predicate = bool;
//
//     fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
//         *predicate
//     }
// }
