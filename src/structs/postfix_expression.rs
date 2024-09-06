use crate::enums::postfix_token::PostfixToken;
use crate::internals::postfix_stack_item::PostfixStackItem;
use crate::traits::predicate_evaluator::PredicateEvaluator;
use crate::{InfixExpression, InfixToken, Operator, Parenthesis};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct PostfixExpression<Predicate> {
    tokens: Vec<PostfixToken<Predicate>>,
}

impl<Predicate: Clone> PostfixExpression<Predicate> {
    #[must_use]
    pub fn from_tokens(tokens: Vec<PostfixToken<Predicate>>) -> Option<Self> {
        Self::are_tokens_valid(&tokens).then(|| Self { tokens })
    }

    pub fn get_tokens(&self) -> Vec<PostfixToken<Predicate>> {
        self.tokens.clone()
    }

    #[must_use]
    pub fn to_infix(self) -> InfixExpression<Predicate> {
        let mut operator_stack: Vec<Option<Operator>> = Vec::new();
        let mut output_stack: Vec<VecDeque<InfixToken<Predicate>>> = Vec::new();

        for token in self.tokens {
            match token {
                PostfixToken::Predicate(p) => {
                    output_stack.push(VecDeque::from([InfixToken::Predicate(p)]));
                    operator_stack.push(None);
                }
                PostfixToken::Operator(op) => {
                    let mut p2 = output_stack.remove(output_stack.len() - 1);
                    let mut p1 = output_stack.remove(output_stack.len() - 1);
                    let op2 = operator_stack.remove(operator_stack.len() - 1);
                    let op1 = operator_stack.remove(operator_stack.len() - 1);

                    for (operator, p) in [(op1, &mut p1), (op2, &mut p2)] {
                        if let Some(operator) = operator {
                            if operator.precedence() < op.precedence() {
                                p.push_front(InfixToken::Parenthesis(Parenthesis::Open));
                                p.push_back(InfixToken::Parenthesis(Parenthesis::Close));
                            }
                        }
                    }

                    let mut v = VecDeque::new();
                    v.extend(p1);
                    v.push_back(InfixToken::Operator(op));
                    v.extend(p2);

                    output_stack.push(v);
                    operator_stack.push(Some(op));
                }
            }
        }

        InfixExpression::from_tokens_unchecked(output_stack.remove(0).into())
    }

    pub fn evaluate(&self, evaluator: &dyn PredicateEvaluator<Predicate = Predicate>) -> bool {
        let mut stack: Vec<PostfixStackItem<Predicate>> = Vec::new();
        for token in &self.tokens {
            match token {
                PostfixToken::Operator(op) => {
                    let mut p2 = stack.remove(stack.len() - 1);
                    let mut p1 = stack.remove(stack.len() - 1);
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
        stack.remove(stack.len() - 1).evaluate(evaluator)
    }

    pub(crate) fn from_tokens_unchecked(tokens: Vec<PostfixToken<Predicate>>) -> Self {
        Self { tokens }
    }

    fn are_tokens_valid(tokens: &[PostfixToken<Predicate>]) -> bool {
        let mut cnt: usize = 0;

        for token in tokens {
            match token {
                PostfixToken::Operator(_) => {
                    if cnt < 2 {
                        return false;
                    }
                    cnt -= 1;
                }
                PostfixToken::Predicate(_) => {
                    cnt += 1;
                }
            }
        }

        cnt == 1
    }
}
