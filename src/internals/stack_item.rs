use crate::traits::rpn_evaluator::RpnEvaluator;

pub(crate) enum StackItem<'a, Predicate> {
    Predicate(&'a Predicate),
    Result(bool),
}

impl<Predicate> StackItem<'_, Predicate> {
    pub(crate) fn evaluate(&self, context: &dyn RpnEvaluator<Predicate=Predicate>) -> bool {
        match self {
            StackItem::Predicate(predicate) => context.evaluate_predicate(predicate),
            StackItem::Result(result) => *result,
        }
    }
}
