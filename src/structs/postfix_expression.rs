use crate::enums::postfix_token::PostfixToken;
use crate::internals::stack_item::StackItem;
use crate::traits::predicate_evaluator::PredicateEvaluator;
use crate::Operator;

pub struct PostfixExpression<Predicate> {
    tokens: Vec<PostfixToken<Predicate>>,
}

impl<Predicate> PostfixExpression<Predicate> {
    #[must_use]
    pub fn from_tokens(tokens: Vec<PostfixToken<Predicate>>) -> Self {
        PostfixExpression { tokens }
    }

    pub fn evaluate(
        &self,
        evaluator: &dyn PredicateEvaluator<Predicate = Predicate>,
    ) -> Option<bool> {
        let mut stack: Vec<StackItem<Predicate>> = Vec::new();
        for token in &self.tokens {
            match token {
                PostfixToken::Operator(op) => {
                    let mut p2 = stack.pop()?;
                    let mut p1 = stack.pop()?;
                    if matches!(p1, StackItem::Predicate(_)) && matches!(p2, StackItem::Result(_)) {
                        std::mem::swap(&mut p1, &mut p2);
                    }
                    let result = match op {
                        Operator::And => p1.evaluate(evaluator) && p2.evaluate(evaluator),
                        Operator::Or => p1.evaluate(evaluator) || p2.evaluate(evaluator),
                    };
                    stack.push(StackItem::Result(result));
                }
                PostfixToken::Predicate(p) => {
                    stack.push(StackItem::Predicate(p));
                }
            }
        }
        Some(stack.pop()?.evaluate(evaluator))
    }
}
