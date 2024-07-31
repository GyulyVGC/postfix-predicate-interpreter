use crate::traits::predicate_evaluator::PredicateEvaluator;

pub(crate) enum StackItem<'a, Predicate> {
    Predicate(&'a Predicate),
    Result(bool),
}

impl<Predicate> StackItem<'_, Predicate> {
    pub(crate) fn evaluate(
        &self,
        evaluator: &dyn PredicateEvaluator<Predicate = Predicate>,
    ) -> bool {
        match self {
            StackItem::Predicate(predicate) => evaluator.evaluate_predicate(predicate),
            StackItem::Result(result) => *result,
        }
    }
}
