use crate::traits::rpn_evaluator::RpnPredicateEvaluator;

pub(crate) enum StackItem<'a, Predicate> {
    Predicate(&'a Predicate),
    Result(bool),
}

impl<Predicate> StackItem<'_, Predicate> {
    pub(crate) fn evaluate(
        &self,
        evaluator: &dyn RpnPredicateEvaluator<Predicate = Predicate>,
    ) -> bool {
        match self {
            StackItem::Predicate(predicate) => evaluator.evaluate_predicate(predicate),
            StackItem::Result(result) => *result,
        }
    }
}
