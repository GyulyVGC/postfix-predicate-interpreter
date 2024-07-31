use crate::internals::infix_stack_item::InfixStackItem;
use crate::{InfixToken, Parenthesis, PostfixExpression, PostfixToken};

pub struct InfixExpression<Predicate> {
    tokens: Vec<InfixToken<Predicate>>,
}

impl<Predicate> InfixExpression<Predicate> {
    #[must_use]
    pub fn from_tokens(tokens: Vec<InfixToken<Predicate>>) -> Self {
        // TODO: verify that the expression is valid
        InfixExpression { tokens }
    }

    #[must_use]
    pub fn to_postfix(self) -> Option<PostfixExpression<Predicate>> {
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
                    if stack.last() != Some(&InfixStackItem::Parenthesis(Parenthesis::Open)) {
                        return None;
                    }
                    stack.pop();
                }
            }
        }

        while let Some(InfixStackItem::Operator(op)) = stack.pop() {
            output_queue.push(PostfixToken::Operator(op));
        }

        if !stack.is_empty() {
            return None;
        }

        Some(PostfixExpression::from_tokens(output_queue))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        InfixExpression, InfixToken, Operator, Parenthesis, PostfixExpression, PostfixToken,
    };

    #[test]
    fn test_infix_to_postfix_successful_1() {
        let infix = InfixExpression::from_tokens(vec![
            InfixToken::Predicate("a"),
            InfixToken::Operator(Operator::And),
            InfixToken::Parenthesis(Parenthesis::Open),
            InfixToken::Predicate("b"),
            InfixToken::Operator(Operator::Or),
            InfixToken::Predicate("c"),
            InfixToken::Parenthesis(Parenthesis::Close),
        ]);

        let postfix = infix.to_postfix().unwrap();
        assert_eq!(
            postfix,
            PostfixExpression::from_tokens(vec![
                PostfixToken::Predicate("a"),
                PostfixToken::Predicate("b"),
                PostfixToken::Predicate("c"),
                PostfixToken::Operator(Operator::Or),
                PostfixToken::Operator(Operator::And),
            ])
        );
    }

    #[test]
    fn test_infix_to_postfix_successful_2() {
        let infix = InfixExpression::from_tokens(vec![
            InfixToken::Predicate("a"),
            InfixToken::Operator(Operator::And),
            InfixToken::Predicate("b"),
            InfixToken::Operator(Operator::Or),
            InfixToken::Predicate("c"),
        ]);

        let postfix = infix.to_postfix().unwrap();
        assert_eq!(
            postfix,
            PostfixExpression::from_tokens(vec![
                PostfixToken::Predicate("a"),
                PostfixToken::Predicate("b"),
                PostfixToken::Operator(Operator::And),
                PostfixToken::Predicate("c"),
                PostfixToken::Operator(Operator::Or),
            ])
        );
    }

    #[test]
    fn test_infix_to_postfix_successful_3() {
        let infix = InfixExpression::from_tokens(vec![InfixToken::Predicate("a")]);

        let postfix = infix.to_postfix().unwrap();
        assert_eq!(
            postfix,
            PostfixExpression::from_tokens(vec![PostfixToken::Predicate("a"),])
        );
    }

    #[test]
    fn test_infix_to_postfix_successful_4() {
        for op in [Operator::And, Operator::Or] {
            let infix = InfixExpression::from_tokens(vec![
                InfixToken::Predicate("a"),
                InfixToken::Operator(op),
                InfixToken::Predicate("b"),
                InfixToken::Operator(op),
                InfixToken::Predicate("c"),
                InfixToken::Operator(op),
                InfixToken::Predicate("d"),
            ]);

            let postfix = infix.to_postfix().unwrap();
            assert_eq!(
                postfix,
                PostfixExpression::from_tokens(vec![
                    PostfixToken::Predicate("a"),
                    PostfixToken::Predicate("b"),
                    PostfixToken::Operator(op),
                    PostfixToken::Predicate("c"),
                    PostfixToken::Operator(op),
                    PostfixToken::Predicate("d"),
                    PostfixToken::Operator(op),
                ])
            );
        }
    }

    #[test]
    fn test_infix_to_postfix_successful_5() {
        let infix = InfixExpression::from_tokens(vec![
            InfixToken::Predicate("a"),
            InfixToken::Operator(Operator::And),
            InfixToken::Parenthesis(Parenthesis::Open),
            InfixToken::Predicate("b"),
            InfixToken::Operator(Operator::Or),
            InfixToken::Predicate("c"),
            InfixToken::Operator(Operator::Or),
            InfixToken::Predicate("d"),
            InfixToken::Operator(Operator::And),
            InfixToken::Parenthesis(Parenthesis::Open),
            InfixToken::Predicate("e"),
            InfixToken::Operator(Operator::Or),
            InfixToken::Predicate("f"),
            InfixToken::Parenthesis(Parenthesis::Close),
            InfixToken::Operator(Operator::Or),
            InfixToken::Predicate("g"),
            InfixToken::Parenthesis(Parenthesis::Close),
            InfixToken::Operator(Operator::Or),
            InfixToken::Predicate("h"),
            InfixToken::Operator(Operator::Or),
            InfixToken::Predicate("i"),
            InfixToken::Operator(Operator::And),
            InfixToken::Predicate("j"),
        ]);

        let postfix = infix.to_postfix().unwrap();
        assert_eq!(
            postfix,
            PostfixExpression::from_tokens(vec![
                PostfixToken::Predicate("a"),
                PostfixToken::Predicate("b"),
                PostfixToken::Predicate("c"),
                PostfixToken::Operator(Operator::Or),
                PostfixToken::Predicate("d"),
                PostfixToken::Predicate("e"),
                PostfixToken::Predicate("f"),
                PostfixToken::Operator(Operator::Or),
                PostfixToken::Operator(Operator::And),
                PostfixToken::Operator(Operator::Or),
                PostfixToken::Predicate("g"),
                PostfixToken::Operator(Operator::Or),
                PostfixToken::Operator(Operator::And),
                PostfixToken::Predicate("h"),
                PostfixToken::Operator(Operator::Or),
                PostfixToken::Predicate("i"),
                PostfixToken::Predicate("j"),
                PostfixToken::Operator(Operator::And),
                PostfixToken::Operator(Operator::Or),
            ])
        );
    }
}
