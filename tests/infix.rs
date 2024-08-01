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
    ])
    .unwrap();

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
    ])
    .unwrap();

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
    let infix = InfixExpression::from_tokens(vec![InfixToken::Predicate("a")]).unwrap();

    let postfix = infix.to_postfix().unwrap();
    assert_eq!(
        postfix,
        PostfixExpression::from_tokens(vec![PostfixToken::Predicate("a"),])
    );
}

#[test]
// (a) --> a
fn test_infix_to_postfix_single_with_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("a"),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]).unwrap();

    let postfix = infix.to_postfix().unwrap();
    assert_eq!(
        postfix,
        PostfixExpression::from_tokens(vec![PostfixToken::Predicate("a"),])
    );
}

#[test]
// (a+b) --> ab+
fn test_infix_to_postfix_simple_with_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("b"),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]).unwrap();

    let postfix = infix.to_postfix().unwrap();
    assert_eq!(
        postfix,
        PostfixExpression::from_tokens(vec![
            PostfixToken::Predicate("a"),
            PostfixToken::Predicate("b"),
            PostfixToken::Operator(Operator::Or),
        ])
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
        ])
        .unwrap();

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
// ((a*(b+c+d*(e+f)+g)+h+i*j)) --> abc+def+*+g+*h+ij*+
fn test_infix_to_postfix_complex() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Parenthesis(Parenthesis::Open),
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
        InfixToken::Parenthesis(Parenthesis::Close),
        InfixToken::Parenthesis(Parenthesis::Close),
    ])
    .unwrap();

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

#[test]
// a*+b [invalid]
fn test_infix_invalid_consecutive_operators() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::And),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("b"),
    ]);
    assert!(infix.is_none());
}

#[test]
// + [invalid]
// * [invalid]
fn test_infix_invalid_only_one_operator() {
    for op in [Operator::And, Operator::Or] {
        let infix = InfixExpression::<u8>::from_tokens(vec![InfixToken::Operator(op)]);
        assert!(infix.is_none());
    }
}

#[test]
// +*+ [invalid]
fn test_infix_invalid_only_operators() {
    let infix = InfixExpression::<u8>::from_tokens(vec![
        InfixToken::Operator(Operator::Or),
        InfixToken::Operator(Operator::And),
        InfixToken::Operator(Operator::Or),
    ]);
    assert!(infix.is_none());
}

#[test]
// ( [invalid]
// ) [invalid]
fn test_infix_invalid_only_one_parenthesis() {
    for paren in [Parenthesis::Open, Parenthesis::Close] {
        let infix = InfixExpression::<u8>::from_tokens(vec![InfixToken::Parenthesis(paren)]);
        assert!(infix.is_none());
    }
}

#[test]
// () [invalid]
fn test_infix_invalid_only_parenthesis() {
    let infix = InfixExpression::<u8>::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}

#[test]
// +a [invalid]
fn test_infix_invalid_starts_with_operator() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("a"),
    ]);
    assert!(infix.is_none());
}

#[test]
// (+a) [invalid]
fn test_infix_invalid_starts_with_parenthesis_and_operator() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("a"),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}

#[test]
// a+ [invalid]
fn test_infix_invalid_ends_with_operator() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::Or),
    ]);
    assert!(infix.is_none());
}
#[test]

// (a+) [invalid]
fn test_infix_invalid_ends_with_operator_and_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}

#[test]
// ab+ [invalid]
fn test_infix_invalid_consecutive_predicates_and_operator() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Predicate("b"),
        InfixToken::Operator(Operator::Or),
    ]);
    assert!(infix.is_none());
}

#[test]
// +ab [invalid]
fn test_infix_invalid_operator_and_consecutive_predicates() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("a"),
        InfixToken::Predicate("b"),
    ]);
    assert!(infix.is_none());
}

#[test]
// ab*c+ [invalid]
fn test_infix_invalid_using_postfix() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Predicate("b"),
        InfixToken::Operator(Operator::And),
        InfixToken::Predicate("c"),
        InfixToken::Operator(Operator::Or),
    ]);
    assert!(infix.is_none());
}

#[test]
// ab [invalid]
fn test_infix_invalid_only_predicates() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Predicate("b"),
    ]);
    assert!(infix.is_none());
}

#[test]
// )a+b( [invalid]
fn test_infix_invalid_swapped_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Close),
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("b"),
        InfixToken::Parenthesis(Parenthesis::Open),
    ]);
    assert!(infix.is_none());
}

#[test]
// (a+b [invalid]
fn test_infix_invalid_missing_close_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("b"),
    ]);
    assert!(infix.is_none());
}

#[test]
// a+b) [invalid]
fn test_infix_invalid_missing_open_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("b"),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}

#[test]
// (a*(b+c) [invalid]
fn test_infix_invalid_more_open_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::And),
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("b"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("c"),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}

#[test]
// a*(b+c)) [invalid]
fn test_infix_invalid_more_close_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::And),
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("b"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("c"),
        InfixToken::Parenthesis(Parenthesis::Close),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}

#[test]
// a(b+c) [invalid]
fn test_infix_invalid_missing_operator_before_open_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("b"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("c"),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}

#[test]
// a*(+b+c) [invalid]
fn test_infix_invalid_operator_after_open_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::And),
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("b"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("c"),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}

#[test]
// a*(b+c)d [invalid]
fn test_infix_invalid_predicate_after_close_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::And),
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("b"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Predicate("c"),
        InfixToken::Parenthesis(Parenthesis::Close),
        InfixToken::Predicate("d"),
    ]);
    assert!(infix.is_none());
}

#[test]
// a*(b+) [invalid]
fn test_infix_invalid_missing_predicate_before_close_parenthesis() {
    let infix = InfixExpression::from_tokens(vec![
        InfixToken::Predicate("a"),
        InfixToken::Operator(Operator::And),
        InfixToken::Parenthesis(Parenthesis::Open),
        InfixToken::Predicate("b"),
        InfixToken::Operator(Operator::Or),
        InfixToken::Parenthesis(Parenthesis::Close),
    ]);
    assert!(infix.is_none());
}
