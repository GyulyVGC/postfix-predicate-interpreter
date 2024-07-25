use crate::enums::rpn_token::RpnToken;
use crate::internals::stack_item::StackItem;
use crate::traits::rpn_evaluator::RpnEvaluator;

pub struct RpnExpression<E: RpnEvaluator> {
    tokens: Vec<RpnToken<E::Predicate>>,
}

impl<E: RpnEvaluator> RpnExpression<E> {
    pub fn from_tokens(tokens: Vec<RpnToken<E::Predicate>>) -> Self {
        RpnExpression { tokens }
    }

    pub fn evaluate(&self, evaluator: &E) -> bool {
        let mut stack: Vec<StackItem<E>> = Vec::new();
        for token in &self.tokens {
            match token {
                RpnToken::Operator(op) => {
                    let p2 = stack.pop().unwrap();
                    let p1 = stack.pop().unwrap();
                    let result = match op {
                        crate::enums::rpn_operator::RpnOperator::And => {
                            p1.evaluate(evaluator) && p2.evaluate(evaluator)
                        }
                        crate::enums::rpn_operator::RpnOperator::Or => {
                            p1.evaluate(evaluator) || p2.evaluate(evaluator)
                        }
                    };
                    stack.push(StackItem::Result(result));
                }
                RpnToken::Predicate(p) => {
                    stack.push(StackItem::Predicate(p));
                }
            }
        }
        stack.pop().unwrap().evaluate(evaluator)
    }
}
