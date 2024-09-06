use crate::internals::infix_stack_item::InfixStackItem;
use crate::{InfixToken, Parenthesis, PostfixExpression, PostfixToken};

#[derive(Debug, PartialEq)]
pub struct InfixExpression<Predicate> {
    tokens: Vec<InfixToken<Predicate>>,
}

impl<Predicate: Clone> InfixExpression<Predicate> {
    #[must_use]
    pub fn from_tokens(tokens: Vec<InfixToken<Predicate>>) -> Option<Self> {
        Self::are_tokens_valid(&tokens).then(|| Self { tokens })
    }

    #[must_use]
    pub fn to_postfix(self) -> PostfixExpression<Predicate> {
        let mut stack: Vec<InfixStackItem> = Vec::new();
        let mut output_queue: Vec<PostfixToken<Predicate>> = Vec::new();

        for token in self.tokens {
            match token {
                InfixToken::Predicate(p) => {
                    output_queue.push(PostfixToken::Predicate(p));
                }
                InfixToken::Operator(op) => {
                    let precedence = op.precedence();
                    while let Some(InfixStackItem::Operator(stack_op)) = stack.last() {
                        if precedence > stack_op.precedence() {
                            break;
                        }
                        output_queue.push(PostfixToken::Operator(*stack_op));
                        stack.pop();
                    }
                    stack.push(InfixStackItem::Operator(op));
                }
                InfixToken::Parenthesis(Parenthesis::Open) => {
                    stack.push(InfixStackItem::Parenthesis(Parenthesis::Open));
                }
                InfixToken::Parenthesis(Parenthesis::Close) => {
                    while let Some(InfixStackItem::Operator(op)) = stack.last() {
                        output_queue.push(PostfixToken::Operator(*op));
                        stack.pop();
                    }
                    // pop the open parenthesis
                    stack.pop();
                }
            }
        }

        while let Some(InfixStackItem::Operator(op)) = stack.pop() {
            output_queue.push(PostfixToken::Operator(op));
        }

        PostfixExpression::from_tokens_unchecked(output_queue)
    }

    pub(crate) fn from_tokens_unchecked(tokens: Vec<InfixToken<Predicate>>) -> Self {
        Self { tokens }
    }

    fn are_tokens_valid(tokens: &[InfixToken<Predicate>]) -> bool {
        let mut operator_stack: Vec<InfixStackItem> = Vec::new();
        let mut predicate_cnt: usize = 0;
        let mut predicate_expected = true;

        for token in tokens {
            match token {
                InfixToken::Predicate(_) => {
                    if !predicate_expected {
                        return false;
                    }
                    predicate_cnt += 1;
                    predicate_expected = false;
                }
                InfixToken::Operator(op) => {
                    operator_stack.push(InfixStackItem::Operator(*op));
                    predicate_expected = true;
                }
                InfixToken::Parenthesis(Parenthesis::Open) => {
                    operator_stack.push(InfixStackItem::Parenthesis(Parenthesis::Open));
                }
                InfixToken::Parenthesis(Parenthesis::Close) => {
                    while let Some(InfixStackItem::Operator(_)) = operator_stack.last() {
                        operator_stack.pop();
                        if predicate_cnt < 2 {
                            return false;
                        }
                        predicate_cnt -= 1;
                    }
                    if operator_stack.last()
                        != Some(&InfixStackItem::Parenthesis(Parenthesis::Open))
                    {
                        return false;
                    }
                    operator_stack.pop();
                }
            }
        }

        while let Some(item) = operator_stack.pop() {
            match item {
                InfixStackItem::Operator(_) => {
                    if predicate_cnt < 2 {
                        return false;
                    }
                    predicate_cnt -= 1;
                }
                InfixStackItem::Parenthesis(_) => {
                    return false;
                }
            }
        }

        predicate_cnt == 1 && operator_stack.is_empty()
    }
}
