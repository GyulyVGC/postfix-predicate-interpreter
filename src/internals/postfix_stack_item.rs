use crate::traits::predicate_evaluator::PredicateEvaluator;

pub(crate) enum PostfixStackItem<'a, Predicate> {
    Predicate(&'a Predicate),
    Result(bool),
}

impl<Predicate> PostfixStackItem<'_, Predicate> {
    pub(crate) async fn evaluate<Reason, Context>(
        &self,
        evaluator: &dyn PredicateEvaluator<
            Predicate = Predicate,
            Reason = Reason,
            Context = Context,
        >,
        reasons: &mut Vec<Reason>,
        context: &Context,
    ) -> bool {
        match self {
            PostfixStackItem::Predicate(predicate) => {
                evaluator.evaluate_predicate_with_reasons_and_context(predicate, reasons, context).await
            }
            PostfixStackItem::Result(result) => *result,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::predicate_evaluator::PredicateEvaluator;

    use super::*;

    struct MyInteger {
        val: i32,
    }

    impl PredicateEvaluator for MyInteger {
        type Predicate = bool;
        type Reason = i32;
        type Context = ();

        fn evaluate_predicate(&self, predicate: &Self::Predicate, _: &()) -> bool {
            if self.val >= 0 {
                *predicate
            } else {
                !*predicate
            }
        }

        fn get_reason(&self, _predicate: &Self::Predicate) -> Self::Reason {
            self.val
        }
    }

    #[test]
    fn test_postfix_stack_item_evaluate() {
        let p1 = false;
        let p2 = true;
        let int1 = MyInteger { val: -1 };
        let int2 = MyInteger { val: 0 };
        let int3 = MyInteger { val: 1 };

        assert!(!PostfixStackItem::Result(p1).evaluate(&int1, &mut Vec::new(), &()));
        assert!(!PostfixStackItem::Result(p1).evaluate(&int2, &mut Vec::new(), &()));
        assert!(!PostfixStackItem::Result(p1).evaluate(&int3, &mut Vec::new(), &()));

        assert!(PostfixStackItem::Result(p2).evaluate(&int1, &mut Vec::new(), &()));
        assert!(PostfixStackItem::Result(p2).evaluate(&int2, &mut Vec::new(), &()));
        assert!(PostfixStackItem::Result(p2).evaluate(&int3, &mut Vec::new(), &()));

        assert!(PostfixStackItem::Predicate(&p1).evaluate(&int1, &mut Vec::new(), &()));
        assert!(!PostfixStackItem::Predicate(&p1).evaluate(&int2, &mut Vec::new(), &()));
        assert!(!PostfixStackItem::Predicate(&p1).evaluate(&int3, &mut Vec::new(), &()));

        assert!(!PostfixStackItem::Predicate(&p2).evaluate(&int1, &mut Vec::new(), &()));
        assert!(PostfixStackItem::Predicate(&p2).evaluate(&int2, &mut Vec::new(), &()));
        assert!(PostfixStackItem::Predicate(&p2).evaluate(&int3, &mut Vec::new(), &()));
    }
}
