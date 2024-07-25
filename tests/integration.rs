use generic_rpn_interpreter::{RpnExpression, RpnOperator, RpnPredicateEvaluator, RpnToken};

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
fn test_rpn() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "5".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::Equal,
        val: "3".to_string(),
    };

    let expr = RpnExpression::from_tokens(vec![
        RpnToken::Predicate(a),
        RpnToken::Predicate(b),
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
