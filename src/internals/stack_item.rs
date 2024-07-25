use crate::traits::rpn_evaluator::RpnEvaluator;

pub(crate) enum StackItem<'a, Ctx: RpnEvaluator> {
    Predicate(&'a Ctx::Predicate),
    Result(bool),
}

impl<Ctx: RpnEvaluator> StackItem<'_, Ctx> {
    pub(crate) fn evaluate(&self, context: &Ctx) -> bool {
        match self {
            StackItem::Predicate(predicate) => context.evaluate_predicate(predicate),
            StackItem::Result(result) => *result,
        }
    }
}
