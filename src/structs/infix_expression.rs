use crate::internals::infix_stack_item::InfixStackItem;
use crate::{InfixToken, Parenthesis, PostfixExpression, PostfixToken};

pub struct InfixExpression<Predicate> {
    tokens: Vec<InfixToken<Predicate>>,
}

impl<Predicate> InfixExpression<Predicate> {
    #[must_use]
    pub fn from_tokens(tokens: Vec<InfixToken<Predicate>>) -> Option<Self> {
        if Self::are_tokens_valid(&tokens) {
            Some(Self { tokens })
        } else {
            None
        }
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

        PostfixExpression::from_tokens(output_queue)
    }

    fn are_tokens_valid(tokens: &[InfixToken<Predicate>]) -> bool {
        let mut operator_stack: Vec<InfixStackItem> = Vec::new();
        let mut predicate_stack: Vec<&Predicate> = Vec::new();
        let mut predicate_expected = true;

        for token in tokens {
            match token {
                InfixToken::Predicate(p) => {
                    if !predicate_expected {
                        return false;
                    }
                    predicate_stack.push(p);
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
                        if predicate_stack.len() < 2 {
                            return false;
                        }
                        predicate_stack.pop();
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
                    if predicate_stack.len() < 2 {
                        return false;
                    }
                    predicate_stack.pop();
                }
                InfixStackItem::Parenthesis(_) => {
                    return false;
                }
            }
        }

        predicate_stack.len() == 1 && operator_stack.is_empty()
    }
}
