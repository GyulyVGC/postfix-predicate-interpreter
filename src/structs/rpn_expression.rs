use crate::enums::rpn_token::RpnToken;
use crate::internals::stack_item::StackItem;
use crate::traits::rpn_evaluator::RpnPredicateEvaluator;
use crate::RpnOperator;

pub struct RpnExpression<Predicate> {
    tokens: Vec<RpnToken<Predicate>>,
}

impl<Predicate> RpnExpression<Predicate> {
    #[must_use]
    pub fn from_tokens(tokens: Vec<RpnToken<Predicate>>) -> Self {
        RpnExpression { tokens }
    }

    pub fn evaluate(
        &self,
        evaluator: &dyn RpnPredicateEvaluator<Predicate = Predicate>,
    ) -> Option<bool> {
        let mut stack: Vec<StackItem<Predicate>> = Vec::new();
        for token in &self.tokens {
            match token {
                RpnToken::Operator(op) => {
                    let mut p2 = stack.pop()?;
                    let mut p1 = stack.pop()?;
                    if matches!(p1, StackItem::Predicate(_)) && matches!(p2, StackItem::Result(_)) {
                        std::mem::swap(&mut p1, &mut p2);
                    }
                    let result = match op {
                        RpnOperator::And => p1.evaluate(evaluator) && p2.evaluate(evaluator),
                        RpnOperator::Or => p1.evaluate(evaluator) || p2.evaluate(evaluator),
                    };
                    stack.push(StackItem::Result(result));
                }
                RpnToken::Predicate(p) => {
                    stack.push(StackItem::Predicate(p));
                }
            }
        }
        Some(stack.pop()?.evaluate(evaluator))
    }
}
