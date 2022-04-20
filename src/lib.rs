/// Holds an expression in a vector of `Tokens` along with the original (white-space-trimmed) expression.
#[derive(Debug, PartialEq)]
pub struct Expression {
    original: String,
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(u64), // floats should be represented as devisions and negative numbers are represented as operations
    Plus,
    Min,
    Prod,
    Dev,
    Left,
    Right,
}

impl Expression {
    /// Creates a new expression from the given string slice.
    /// Tokenizes it and stores it in postfix form using the [Shunting-Yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
    ///
    /// # Panics
    ///
    /// Panics when the given expression contains invalid tokens.
    ///
    /// # Todo
    /// Check if expression has balanced brackets.
    pub fn new(expr: &str) -> Self {
        let expr: String = expr.chars().filter(|ch| *ch != ' ').collect();

        let tokens = Self::tokenize(&expr);
        // println!("After tokenize: {:?}", tokens);
        let tokens = Self::to_post(tokens);
        // println!("After to_post: {:?}", tokens);

        Self {
            original: expr.to_owned(),
            tokens,
        }
    }

    fn tokenize(expr: &str) -> Vec<Token> {
        // tokenize the string slice and
        // add multiplication signs where needed
        let mut iter = expr.chars().peekable();

        let mut tokens: Vec<Token> = Vec::new();

        while let Some(ch) = iter.next() {
            match ch {
                '0'..='9' => {
                    let mut curr_number = String::from(ch);

                    // check if number is longer than one char
                    loop {
                        match iter.peek() {
                            Some('0'..='9') => curr_number.push(iter.next().unwrap()),
                            _ => break,
                        }
                    }

                    tokens.push(Token::Number(curr_number.parse().unwrap()));

                    // check if next char needs to be a multiplication
                    match iter.peek() {
                        Some('(') => {
                            tokens.push(Token::Prod);
                        },
                        _ => (),
                    }
                },
                '*' => tokens.push(Token::Prod),
                '/' => tokens.push(Token::Dev),
                '+' => tokens.push(Token::Plus),
                '-' => {
                    // check if it is a multiplication with -1
                    let mult = if let Some(next) = iter.peek() {
                        match next {
                            '(' => true,
                            _ => false,
                        }
                    } else {
                        false
                    };

                    if !mult {
                        tokens.push(Token::Min);
                    } else {    // '-' before '(' is seen as '...+(0-1)*(...)' because numbers cannot be stored negative
                        // TODO: Store negative numbers
                        tokens.push(Token::Plus);
                        tokens.push(Token::Left);
                        tokens.push(Token::Number(0));
                        tokens.push(Token::Min);
                        tokens.push(Token::Number(1));
                        tokens.push(Token::Right);
                        tokens.push(Token::Prod);
                    }
                },
                '(' => tokens.push(Token::Left),
                ')' => tokens.push(Token::Right),
                other => panic!("The expression '{}' is not valid, because token '{}' is not a supported token.", expr, other),
            }
        }

        tokens
    }

    /// Converts vector of tokens to vector of tokens in postfix form using the [Shunting-Yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
    fn to_post(original: Vec<Token>) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        for token in original.into_iter() {
            match token {
                Token::Number(_) => tokens.push(token),
                Token::Left => operator_stack.push(token),
                Token::Right => {
                    while !operator_stack.is_empty() {
                        let top = operator_stack.pop().unwrap();

                        match top {
                            Token::Left => break,
                            other => tokens.push(other),
                        }
                    }
                }
                operator => {
                    while !operator_stack.is_empty() {
                        let top = operator_stack.last().unwrap();

                        if *top == Token::Left {
                            break;
                        }

                        if Token::precedence(top) >= Token::precedence(&operator) {
                            tokens.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }

                    operator_stack.push(operator);
                }
            }
        }

        // empty the operator_stack
        while !operator_stack.is_empty() {
            tokens.push(operator_stack.pop().unwrap());
        }

        tokens
    }

    /// Evaluates the expression and returns the result as an `f64`
    pub fn evaluate(&self) -> f64 {
        let mut stack: Vec<f64> = Vec::new();

        for token in self.tokens.iter() {
            match token {
                Token::Number(n) => stack.push(n.clone() as f64),
                operator => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    match operator {
                        Token::Prod => stack.push((b * a) as f64),
                        Token::Dev => stack.push((b / a) as f64),
                        Token::Plus => stack.push((b + a) as f64),
                        Token::Min => stack.push((b - a) as f64),
                        other => panic!(
                            "Tried to use '{:?}' as ann operator when evaluating.",
                            other
                        ),
                    }
                }
            }
        }

        return stack.pop().unwrap();
    }

    /// Returns the original white space trimmed expression as `&str`.
    pub fn as_str(&self) -> &str {
        &self.original
    }
}

impl Token {
    fn precedence(&self) -> usize {
        match *self {
            Self::Prod | Self::Dev => 1,
            Self::Plus | Self::Min => 0,
            _ => panic!("Precedence of token '{:?}' cannot be found.", self),
        }
    }
}

#[cfg(test)]
mod token_tests {
    use super::Token;

    #[test]
    fn precedence_test() {
        assert!(Token::precedence(&Token::Prod) == Token::precedence(&Token::Dev));
        assert!(Token::precedence(&Token::Prod) > Token::precedence(&Token::Plus));
        assert!(Token::precedence(&Token::Prod) > Token::precedence(&Token::Min));
        assert!(Token::precedence(&Token::Dev) > Token::precedence(&Token::Plus));
        assert!(Token::precedence(&Token::Dev) > Token::precedence(&Token::Min));
        assert!(Token::precedence(&Token::Plus) == Token::precedence(&Token::Min));
    }
}

#[cfg(test)]
mod expression_tests {
    use super::Expression;
    use super::Token::*;

    #[test]
    fn tokenize() {
        let tokens = Expression::tokenize("12-(13+7)*3");
        let wanted_tokens = vec![
            Number(12),
            Plus,
            Left,
            Number(0),
            Min,
            Number(1),
            Right,
            Prod,
            Left,
            Number(13),
            Plus,
            Number(7),
            Right,
            Prod,
            Number(3),
        ];

        assert_eq!(wanted_tokens, tokens);
    }

    #[test]
    fn to_post() {
        let tokens = Expression::to_post(Expression::tokenize("133+(15-(125/3)+1)"));
        let wanted_tokens = vec![
            Number(133),
            Number(15),
            Number(0),
            Number(1),
            Min,
            Number(125),
            Number(3),
            Dev,
            Prod,
            Plus,
            Number(1),
            Plus,
            Plus,
        ];

        assert_eq!(wanted_tokens, tokens);
    }

    #[test]
    fn new_expression() {
        let expression = Expression::new("125-(145*9+3-2(12/3))-2");
        let wanted_expression = Expression {
            original: "125-(145*9+3-2(12/3))-2".to_owned(),
            tokens: vec![
                Number(125),
                Number(0),
                Number(1),
                Min,
                Number(145),
                Number(9),
                Prod,
                Number(3),
                Plus,
                Number(2),
                Number(12),
                Number(3),
                Dev,
                Prod,
                Min,
                Prod,
                Plus,
                Number(2),
                Min,
            ],
        };

        assert_eq!(wanted_expression, expression);
    }

    #[test]
    fn evaluate_expression() {
        let result = Expression::new("125-(145*9+3-2(12/3))-2").evaluate();
        let wanted_result = -1177.0;

        assert_eq!(result, wanted_result);
    }
}
