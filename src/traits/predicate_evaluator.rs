pub trait PredicateEvaluator {
    type Predicate;
    type Reason;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool;

    fn get_reason(&self, predicate: &Self::Predicate) -> Self::Reason;

    fn evaluate_predicate_with_reasons(
        &self,
        predicate: &Self::Predicate,
        reasons: &mut Vec<Self::Reason>,
    ) -> bool {
        let res = self.evaluate_predicate(predicate);

        if res {
            reasons.push(self.get_reason(predicate));
        } else {
            reasons.clear();
        }

        res
    }
}

// impl PredicateEvaluator for () {
//     type Predicate = bool;
//
//     fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
//         *predicate
//     }
// }
