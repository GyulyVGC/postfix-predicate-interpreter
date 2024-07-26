use rpn_predicate_interpreter::{RpnExpression, RpnOperator, RpnPredicateEvaluator, RpnToken};

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

impl RpnPredicateEvaluator for MyInteger {
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

impl RpnPredicateEvaluator for MyReal {
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
// a+b --> ab+
fn test_rpn_simple() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "33".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::LowerThan,
        val: "10".to_string(),
    };

    let expr = RpnExpression::from_tokens(vec![
        RpnToken::Predicate(a),
        RpnToken::Predicate(b),
        RpnToken::Operator(RpnOperator::Or),
    ]);

    assert!(!expr.evaluate(&MyInteger { val: 34 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 33 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 12 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 11 }).unwrap());
    assert!(!expr.evaluate(&MyInteger { val: 10 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 9 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 8 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 7 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 6 }).unwrap());
}

#[test]
// a+b*(c+d+e*(f+g)) --> abcd+efg+*+*+
fn test_rpn_complex() {
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

    let expr = RpnExpression::from_tokens(vec![
        RpnToken::Predicate(a),
        RpnToken::Predicate(b),
        RpnToken::Predicate(c),
        RpnToken::Predicate(d),
        RpnToken::Operator(RpnOperator::Or),
        RpnToken::Predicate(e),
        RpnToken::Predicate(f),
        RpnToken::Predicate(g),
        RpnToken::Operator(RpnOperator::Or),
        RpnToken::Operator(RpnOperator::And),
        RpnToken::Operator(RpnOperator::Or),
        RpnToken::Operator(RpnOperator::And),
        RpnToken::Operator(RpnOperator::Or),
    ]);

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
fn test_many_and() {
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

    let expr = RpnExpression::from_tokens(vec![
        RpnToken::Predicate(a),
        RpnToken::Predicate(b),
        RpnToken::Operator(RpnOperator::And),
        RpnToken::Predicate(c),
        RpnToken::Operator(RpnOperator::And),
        RpnToken::Predicate(d),
        RpnToken::Operator(RpnOperator::And),
    ]);

    assert!(!expr.evaluate(&MyInteger { val: 7 }).unwrap());
    assert!(expr.evaluate(&MyInteger { val: 1 }).unwrap());

    assert!(!expr.evaluate(&MyReal { val: 7.0 }).unwrap());
    assert!(expr.evaluate(&MyReal { val: 1.0 }).unwrap());
}

#[test]
// a+b+c+d --> ab+c+d+
fn test_many_or() {
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

    let expr = RpnExpression::from_tokens(vec![
        RpnToken::Predicate(a),
        RpnToken::Predicate(b),
        RpnToken::Operator(RpnOperator::Or),
        RpnToken::Predicate(c),
        RpnToken::Operator(RpnOperator::Or),
        RpnToken::Predicate(d),
        RpnToken::Operator(RpnOperator::Or),
    ]);

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
