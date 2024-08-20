# postfix-predicate-interpreter
A postfix expression converter and interpreter for generic boolean predicates.

> [!NOTE]
> 
> <i>
> Postfix representation, also known as Reverse Polish Notation (RPN),
> is an alternative way to describe mathematical expressions.<br/>
> While the standard representation (infix) uses operators between operands,
> the postfix representation uses operators after operands.<br/>
> Expressions in postfix form are easier and less expensive to evaluate than standard infix expressions:
> they can be evaluated linearly left-to-right, since this notation removes the concept of operator precedence and the need for parenthesis. 
> </i>

## Features
- Instantiate an infix expression made of generic boolean predicates
- Instantiate a postfix expression made of generic boolean predicates
- Convert an infix expression to a postfix expression
- Convert a postfix expression to an infix expression
- Evaluate a postfix expression

## Usage
Let's say you want to evaluate the infix expression `A AND (B OR C)`.

We can instantiate the infix expression as follows:
``` rust
let infix = InfixExpression::from_tokens(vec![
    InfixToken::Predicate("A"),
    InfixToken::Operator(Operator::And),
    InfixToken::Parenthesis(Parenthesis::Open),
    InfixToken::Predicate("B"),
    InfixToken::Operator(Operator::Or),
    InfixToken::Predicate("C"),
    InfixToken::Parenthesis(Parenthesis::Close),
])
.unwrap();
```

We can then convert the infix expression to the equivalent postfix expression `A B C OR AND`:
``` rust
let postfix = infix.to_postfix();
assert_eq!(
    postfix,
    PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate("a"),
        PostfixToken::Predicate("b"),
        PostfixToken::Predicate("c"),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Operator(Operator::And),
    ])
    .unwrap()
);
```

Finally, we can evaluate the postfix expression:
``` rust
postfix.evaluate(&predicate_evaluator);
```

Note that predicates are represented as letters in this example (`A`, `B`, `C`),
but they can be any type that resolves to either true or false given a predicate evaluator.<br/>
The predicate evaluator is an object of a type implementing the `PredicateEvaluator` trait;
such an object is passed as argument to the `evaluate` method, making it possible to solve the expression.

> [!NOTE]
> 
> In the scope of this library, an expression can't be evaluated without a predicate evaluator.<br/>
> Aim of the library is in fact to evaluate expressions whose results depend on an external context,
> represented by the value of the predicate evaluator.
    