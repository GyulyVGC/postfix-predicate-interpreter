use crate::traits::predicate_evaluator::PredicateEvaluator;

pub(crate) enum PostfixStackItem<'a, Predicate> {
    Predicate(&'a Predicate),
    Result(bool),
}

impl<Predicate> PostfixStackItem<'_, Predicate> {
    pub(crate) fn evaluate(
        &self,
        evaluator: &dyn PredicateEvaluator<Predicate = Predicate>,
    ) -> bool {
        match self {
            PostfixStackItem::Predicate(predicate) => evaluator.evaluate_predicate(predicate),
            PostfixStackItem::Result(result) => *result,
        }
    }
}
