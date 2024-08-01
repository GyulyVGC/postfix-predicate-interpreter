use crate::traits::predicate_evaluator::PredicateEvaluator;

pub(crate) enum PostfixStackItem<'a, Predicate> {
    Predicate(&'a Predicate),
    Result(bool),
}

impl<Predicate> PostfixStackItem<'_, Predicate> {
    pub(crate) fn evaluate(
        &self,
        evaluator: &dyn PredicateEvaluator<Predicate = Predicate>,
    ) -> bool {
        match self {
            PostfixStackItem::Predicate(predicate) => evaluator.evaluate_predicate(predicate),
            PostfixStackItem::Result(result) => *result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::predicate_evaluator::PredicateEvaluator;

    struct MyInteger {
        val: i32,
    }

    impl PredicateEvaluator for MyInteger {
        type Predicate = bool;

        fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
            if self.val >= 0 {
                *predicate
            } else {
                !*predicate
            }
        }
    }

    #[test]
    fn test_postfix_stack_item_evaluate() {
        let p1 = false;
        let p2 = true;
        let int1 = MyInteger { val: -1 };
        let int2 = MyInteger { val: 0 };
        let int3 = MyInteger { val: 1 };

        assert!(!PostfixStackItem::Result(p1).evaluate(&int1));
        assert!(!PostfixStackItem::Result(p1).evaluate(&int2));
        assert!(!PostfixStackItem::Result(p1).evaluate(&int3));

        assert!(PostfixStackItem::Result(p2).evaluate(&int1));
        assert!(PostfixStackItem::Result(p2).evaluate(&int2));
        assert!(PostfixStackItem::Result(p2).evaluate(&int3));

        assert!(PostfixStackItem::Predicate(&p1).evaluate(&int1));
        assert!(!PostfixStackItem::Predicate(&p1).evaluate(&int2));
        assert!(!PostfixStackItem::Predicate(&p1).evaluate(&int3));

        assert!(!PostfixStackItem::Predicate(&p2).evaluate(&int1));
        assert!(PostfixStackItem::Predicate(&p2).evaluate(&int2));
        assert!(PostfixStackItem::Predicate(&p2).evaluate(&int3));
    }
}
