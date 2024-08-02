use crate::enums::postfix_token::PostfixToken;
use crate::internals::postfix_stack_item::PostfixStackItem;
use crate::traits::predicate_evaluator::PredicateEvaluator;
use crate::{InfixExpression, Operator};

#[derive(Debug, PartialEq)]
pub struct PostfixExpression<Predicate> {
    tokens: Vec<PostfixToken<Predicate>>,
}

impl<Predicate> PostfixExpression<Predicate> {
    #[must_use]
    pub fn from_tokens(tokens: Vec<PostfixToken<Predicate>>) -> Option<Self> {
        Self::are_tokens_valid(&tokens).then(|| Self { tokens })
    }

    #[must_use]
    pub fn to_infix(self) -> Option<InfixExpression<Predicate>> {
        todo!("postfix to infix conversion");
    }

    pub fn evaluate(
        &self,
        evaluator: &dyn PredicateEvaluator<Predicate = Predicate>,
    ) -> Option<bool> {
        let mut stack: Vec<PostfixStackItem<Predicate>> = Vec::new();
        for token in &self.tokens {
            match token {
                PostfixToken::Operator(op) => {
                    let mut p2 = stack.pop()?;
                    let mut p1 = stack.pop()?;
                    if matches!(p1, PostfixStackItem::Predicate(_))
                        && matches!(p2, PostfixStackItem::Result(_))
                    {
                        std::mem::swap(&mut p1, &mut p2);
                    }
                    let result = match op {
                        Operator::And => p1.evaluate(evaluator) && p2.evaluate(evaluator),
                        Operator::Or => p1.evaluate(evaluator) || p2.evaluate(evaluator),
                    };
                    stack.push(PostfixStackItem::Result(result));
                }
                PostfixToken::Predicate(p) => {
                    stack.push(PostfixStackItem::Predicate(p));
                }
            }
        }
        Some(stack.pop()?.evaluate(evaluator))
    }

    pub(crate) fn from_tokens_unchecked(tokens: Vec<PostfixToken<Predicate>>) -> Self {
        Self { tokens }
    }

    fn are_tokens_valid(tokens: &[PostfixToken<Predicate>]) -> bool {
        todo!("verify that the expression is valid");
    }
}
