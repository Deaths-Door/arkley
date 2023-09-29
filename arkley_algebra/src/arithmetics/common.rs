use std::collections::BTreeMap;
use num_notation::{
    Number,
    fraction::Signed
};
use crate::{Expression,Term,Variables,ArithmeticOperation};

impl Term {
    pub(in crate::arithmetics) fn is_combinable_with(&self,other : &Self) -> bool {
        self.variables == other.variables
    }

    pub(in crate::arithmetics) fn force_add_terms(self,other : Term) -> Self {
        let coefficient = self.coefficient + other.coefficient;
        let variables = self.variables;
        Term::new_with_variable(coefficient,variables)
    }
}

/// Used to combine terms like 2x + x into 3x 
impl Expression { 
    /// Collects all terms of addition (+) or subtraction (-) variants into 'treemap'
    ///
    /// # Returns
    ///
    /// An optional Expression representing the result of combining terms from nested (Nested), multiplication (*),
    /// and division (/) variants.
    fn collect_terms(self,treemap : &mut BTreeMap<Variables,Number>) -> Option<Expression> {
        match self {
            Expression::Nested(inner) => Some(Expression::new_nested(inner.combine_terms())),
            Expression::Term(term) => {
                treemap.entry(term.variables)
                    .and_modify(|value| *value += term.coefficient.clone())
                    .or_insert(term.coefficient);
                None
            }
            Expression::Binary { operation , left , right} if operation == ArithmeticOperation::Plus => {
                let lexpr = left.collect_terms(treemap);
                let rexpr = right.collect_terms(treemap);

                match (lexpr,rexpr) {
                    (None,None) => None,
                    (Some(expr),None) | (None,Some(expr)) => Some(expr),
                    (Some(expr1),Some(expr2)) => Some(Expression::new_binary(operation,expr1,expr2))
                }   
            },

            Expression::Binary { operation , left , right} if operation == ArithmeticOperation::Minus => {
                let lexpr = left.collect_terms(treemap);
                let rexpr = if let Expression::Term(term) = *right {
                    treemap.entry(term.variables)
                        .and_modify(|value| *value -= term.coefficient.clone()) // as +- equals -
                        .or_insert(-term.coefficient); // as operations is - so -number
                    None
                } else { right.collect_terms(treemap) };

                match (lexpr,rexpr) {
                    (None,None) => None,
                    (Some(expr),None) | (None,Some(expr)) => Some(expr),
                    (Some(expr1),Some(expr2)) => Some(Expression::new_binary(operation,expr1,expr2))
                }  
            },
            Expression::Binary { operation , left , right} => {
                let lexpr = left.combine_terms(); 
                let rexpr = right.combine_terms(); 
                Some(Expression::new_binary(operation,lexpr,rexpr))
            },
        }
    }

    /// Reconstructs the expression based on grouped terms and an optional nested expression.
    ///
    /// # Returns
    ///
    /// The reconstructed expression.
    fn reconstruct_expression(terms : BTreeMap<Variables,Number>,nested_expr : Option<Expression>) -> Self {
        let mut expression : Expression = Term::new(Number::Decimal(0.0)).into();

        for (variables,coefficient) in terms.into_iter() {
            let (sign,term) = match coefficient.is_positive() {
                true => (ArithmeticOperation::Plus,Term::new_with_variable(coefficient,variables)),
                // If the coefficient is negative (-coefficient), the sign can be '-', but the number itself is positive. 
                // For example, -3 represents a negative number, whereas --3 is not equal to -3; it represents a positive number.
                false => (ArithmeticOperation::Minus,Term::new_with_variable(-coefficient,variables))
            };

            expression = Expression::new_binary(sign,expression,term.into());
        }

        if let Some(nested) = nested_expr {
            expression = Expression::new_binary(ArithmeticOperation::Plus,expression,nested);
        }
        
        expression
    }

    /// Combines terms within the expression.
    ///
    /// # Returns
    ///
    /// The expression with combined terms.
    pub(in crate::arithmetics) fn combine_terms(self) -> Expression {
        let mut treemap = BTreeMap::new();

        let nested_expr = self.collect_terms(&mut treemap);
    
        Self::reconstruct_expression(treemap,nested_expr)
    }
}