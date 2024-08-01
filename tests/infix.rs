use rpn_predicate_interpreter::{
    InfixExpression, InfixToken, Operator, Parenthesis, PostfixExpression, PostfixToken,
};

#[test]
// a*(b+c) --> abc+*
fn test_infix_to_postfix_parenthesis() {
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
// a*b+c --> ab*c+
fn test_infix_to_postfix_plain() {
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
// a --> a
fn test_infix_to_postfix_single() {
    let infix = InfixExpression::from_tokens(vec![InfixToken::Predicate("a")]);

    let postfix = infix.to_postfix().unwrap();
    assert_eq!(
        postfix,
        PostfixExpression::from_tokens(vec![PostfixToken::Predicate("a"),])
    );
}

#[test]
// a*b*c*d --> ab*c*d*
// a+b+c+d --> ab+c+d+
fn test_infix_to_postfix_and_or() {
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
// a*(b+c+d*(e+f)+g)+h+i*j --> abc+def+*+g+*h+ij*+
fn test_infix_to_postfix_complex() {
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
