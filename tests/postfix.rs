use rpn_predicate_interpreter::{Operator, PostfixExpression, PostfixToken, PredicateEvaluator};

struct Predicate {
    condition: PredicateCondition,
    val: String,
}

enum PredicateCondition {
    Equal,
    NotEqual,
    GreaterThan,
    LowerThan,
}

struct MyInteger {
    val: i32,
}

struct MyReal {
    val: f32,
}

impl PredicateEvaluator for MyInteger {
    type Predicate = Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
        match predicate.condition {
            PredicateCondition::Equal => self.val == predicate.val.parse().unwrap(),
            PredicateCondition::NotEqual => self.val != predicate.val.parse().unwrap(),
            PredicateCondition::GreaterThan => self.val > predicate.val.parse().unwrap(),
            PredicateCondition::LowerThan => self.val < predicate.val.parse().unwrap(),
        }
    }
}

impl PredicateEvaluator for MyReal {
    type Predicate = Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
        match predicate.condition {
            PredicateCondition::Equal => self.val == predicate.val.parse().unwrap(),
            PredicateCondition::NotEqual => self.val != predicate.val.parse().unwrap(),
            PredicateCondition::GreaterThan => self.val > predicate.val.parse().unwrap(),
            PredicateCondition::LowerThan => self.val < predicate.val.parse().unwrap(),
        }
    }
}

#[test]
// a --> a
fn test_postfix_evaluate_single() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "33".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![PostfixToken::Predicate(a)]).unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 34 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 33 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 12 }).unwrap());

    assert!(!expr.evaluate(&MyReal { val: 34.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 33.0 }).unwrap());
    assert!(!expr.evaluate(&MyReal { val: 12.0 }).unwrap());
}

#[test]
// a+b --> ab+
fn test_postfix_evaluate_simple() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "33".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::LowerThan,
        val: "10".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate(a),
        PostfixToken::Predicate(b),
        PostfixToken::Operator(Operator::Or),
    ])
    .unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 34 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 33 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 12 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 11 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 10 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 9 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 8 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 7 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 6 }).unwrap());

    assert!(!expr.evaluate(&MyReal { val: 34.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 33.0 }).unwrap());
    assert!(!expr.evaluate(&MyReal { val: 12.0 }).unwrap());
    assert!(!expr.evaluate(&MyReal { val: 11.0 }).unwrap());
    assert!(!expr.evaluate(&MyReal { val: 10.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 9.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 8.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 7.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 6.0 }).unwrap());
}

#[test]
// a+b*(c+d+e*(f+g)) --> abcd+efg+*+*+
fn test_postfix_evaluate_complex() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "5".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::Equal,
        val: "3".to_string(),
    };
    let c = Predicate {
        condition: PredicateCondition::NotEqual,
        val: "4".to_string(),
    };
    let d = Predicate {
        condition: PredicateCondition::GreaterThan,
        val: "6".to_string(),
    };
    let e = Predicate {
        condition: PredicateCondition::LowerThan,
        val: "9".to_string(),
    };
    let f = Predicate {
        condition: PredicateCondition::Equal,
        val: "7".to_string(),
    };
    let g = Predicate {
        condition: PredicateCondition::NotEqual,
        val: "8".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate(a),
        PostfixToken::Predicate(b),
        PostfixToken::Predicate(c),
        PostfixToken::Predicate(d),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Predicate(e),
        PostfixToken::Predicate(f),
        PostfixToken::Predicate(g),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Operator(Operator::Or),
    ])
    .unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 7 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 6 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 5 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 4 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 3 }).unwrap());

    assert!(!expr.evaluate(&MyReal { val: 7.0 }).unwrap());
    assert!(!expr.evaluate(&MyReal { val: 6.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 5.0 }).unwrap());
    assert!(!expr.evaluate(&MyReal { val: 4.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 3.0 }).unwrap());
}

#[test]
// a*b*c*d --> ab*c*d*
fn test_postfix_evaluate_many_and() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "1".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::NotEqual,
        val: "2".to_string(),
    };
    let c = Predicate {
        condition: PredicateCondition::NotEqual,
        val: "3".to_string(),
    };
    let d = Predicate {
        condition: PredicateCondition::LowerThan,
        val: "4".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate(a),
        PostfixToken::Predicate(b),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Predicate(c),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Predicate(d),
        PostfixToken::Operator(Operator::And),
    ])
    .unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 7 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 1 }).unwrap());

    assert!(!expr.evaluate(&MyReal { val: 7.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 1.0 }).unwrap());
}

#[test]
// a+b+c+d --> ab+c+d+
fn test_postfix_evaluate_many_or() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "1".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::Equal,
        val: "2".to_string(),
    };
    let c = Predicate {
        condition: PredicateCondition::Equal,
        val: "3".to_string(),
    };
    let d = Predicate {
        condition: PredicateCondition::Equal,
        val: "4".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate(a),
        PostfixToken::Predicate(b),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Predicate(c),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Predicate(d),
        PostfixToken::Operator(Operator::Or),
    ])
    .unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 0 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 1 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 2 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 3 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 4 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 5 }).unwrap());

    assert!(!expr.evaluate(&MyReal { val: 0.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 1.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 2.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 3.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 4.0 }).unwrap());
    assert!(!expr.evaluate(&MyReal { val: 5.0 }).unwrap());
}
