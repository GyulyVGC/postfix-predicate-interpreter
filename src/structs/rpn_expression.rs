use crate::enums::rpn_token::RpnToken;
use crate::internals::stack_item::StackItem;
use crate::traits::rpn_evaluator::RpnEvaluator;

pub struct RpnExpression<Predicate> {
    tokens: Vec<RpnToken<Predicate>>,
}

impl<Predicate> RpnExpression<Predicate> {
    pub fn from_tokens(tokens: Vec<RpnToken<Predicate>>) -> Self {
        RpnExpression { tokens }
    }

    pub fn evaluate(&self, evaluator: &dyn RpnEvaluator<Predicate=Predicate>) -> bool {
        let mut stack: Vec<StackItem<Predicate>> = Vec::new();
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
