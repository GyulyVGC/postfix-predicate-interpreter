pub trait PredicateEvaluator {
    type Predicate;
    type Reason;
    type Context;

    fn evaluate_predicate(&self, _predicate: &Self::Predicate, context: &Self::Context) -> bool;

    fn get_reason(&self, predicate: &Self::Predicate) -> Self::Reason;

    fn is_blacklisted(&self) -> bool {
        false
    }

    fn get_remote_ip(&self) -> String {
        String::new()
    }

    fn evaluate_predicate_with_reasons_and_context(
        &self,
        predicate: &Self::Predicate,
        reasons: &mut Vec<Self::Reason>,
        context: &Self::Context,
    ) -> bool {
        let res = self.evaluate_predicate(predicate, context);

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
