use std::collections::{BTreeMap, HashMap};
use num_notation::{
    Number,
    fraction::Signed,
};
use crate::{Expression,Term,Variables,ArithmeticOperation, Function};

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
    /// `None` = No expr left
    fn collect_terms(self,term_map : &mut BTreeMap<Variables,Number>,fn_map : &mut HashMap<Function,i16>) -> Option<Expression> {
        match self {
            Self::Nested(inner) => Some(Expression::new_nested(inner.combine_terms())),
            Self::Term(term) => {
                term_map.entry(term.variables)
                    .and_modify(|value| *value += term.coefficient.clone())
                    .or_insert(term.coefficient);
                None
            }
            Self::Function(func) => {
                fn_map.entry(func)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                None
            },
            Self::Binary { operation , left , right} if operation == ArithmeticOperation::Plus => {
                let lexpr = left.collect_terms(term_map,fn_map);
                let rexpr = right.collect_terms(term_map,fn_map);

                match (lexpr,rexpr) {
                    (None,None) => None,
                    (Some(expr),None) | (None,Some(expr)) => Some(expr),
                    (Some(expr1),Some(expr2)) => Some(Expression::new_binary(operation,expr1,expr2))
                }   
            },

            Self::Binary { operation , left , right} if operation == ArithmeticOperation::Minus => {
                let lexpr = left.collect_terms(term_map,fn_map);
                let rexpr = match *right {
                    Self::Term(term) => {
                        term_map.entry(term.variables)
                            .and_modify(|value| *value -= term.coefficient.clone()) // as +- equals -
                            .or_insert(-term.coefficient); // as operations is - so -number
                        None
                    },
                    Self::Function(func) => {
                        fn_map.entry(func)
                            .and_modify(|value| *value -= 1)
                            .or_insert(-1);
                        None
                    },
                    _ => right.collect_terms(term_map,fn_map),
                };

                match (lexpr,rexpr) {
                    (None,None) => None,
                    (Some(expr),None) | (None,Some(expr)) => Some(expr),
                    (Some(expr1),Some(expr2)) => Some(Expression::new_binary(operation,expr1,expr2))
                }  
            },
            Self::Binary { operation , left , right} => {
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
    fn reconstruct_expression(
        terms : BTreeMap<Variables,Number>,
        functions : HashMap<Function,i16>,
        nested_expr : Option<Expression>
    ) -> Self {  
        let mut expression : Expression = Term::new(Number::Decimal(0.0)).into();

        expression = expression.join_functions_into_expression(functions)            // |
            .join_terms_into_expression(terms)                          // |
            .join_nested_expression_into_expression(nested_expr)   ;

        println!("reconstruct_expression with = {expression}");

      //  expression.remove_leftmost_zero()

      expression
    /*                                                                         // |
        expression.join_functions_into_expression(functions)            // |
            .join_terms_into_expression(terms)                          // |
            .join_nested_expression_into_expression(nested_expr)        // |
            .remove_leftmost_zero() // Removes -----------------------------*/
    }

    /// Combines terms within the expression.
    ///
    /// # Returns
    ///
    /// The expression with combined terms.
    pub(in crate::arithmetics) fn combine_terms(self) -> Expression {
        let mut term_map = BTreeMap::new();

        let mut fn_map = HashMap::new();

        let nested_expr = self.collect_terms(&mut term_map,&mut fn_map);
    
        Self::reconstruct_expression(term_map,fn_map,nested_expr)
    }
}

impl Expression {
        
    fn join_functions_into_expression(mut self,functions : HashMap<Function,i16>) -> Self {
        let mut functions_sorted_by_count : Vec<(Function,i16)> = functions.into_iter().collect();
        functions_sorted_by_count.sort_by_key(|(_,key)| *key);

        for (_function,count) in functions_sorted_by_count {
            let function : Expression = _function.into();
            if count == 1 {
                self = Expression::new_plus(self, function);
                continue;
            }

            self = match count.is_positive() {
                true => Expression::new_plus(
                    self, 
                    Expression::new_mal(
                        count.into(), 
                        function
                    )
                ),
                false => Expression::new_minus(
                    self, 
                    Expression::new_mal(
                        (-count).into(), 
                        function
                    )
                )
            }
        }

        self
    }

    fn join_terms_into_expression(mut self,terms : BTreeMap<Variables,Number>) -> Self {
        for (variables,coefficient) in terms {
            self = match coefficient.is_positive() {
                true => Expression::new_plus(self, Term::new_with_variable(coefficient,variables).into()),
                // If the coefficient is negative (-coefficient), the sign can be '-', but the number itself is positive. 
                // For example, -3 represents a negative number, whereas --3 is not equal to -3; it represents a positive number.
                false => Expression::new_minus(self, Term::new_with_variable(-coefficient,variables).into()),
            }
        }

        self
    }
    
    fn join_nested_expression_into_expression(self,nested_expr : Option<Expression>) -> Self { 
        if let Some(nested) = nested_expr {
            return Self::new_plus(self,nested);
        }
        
        self
    }

    fn remove_leftmost_zero(self) -> Self {
        println!("before removed zero = {self}");

        let returned = match self {
            // Handles cases like 0 - 3x where the 0 is remove and so is the sign hence we negate leftmost term to 'keep' the sign
            Expression::Binary { ref left, right, operation } 
                if left.is_removable() => match operation == ArithmeticOperation::Minus {
                    true => right.negate_leftmost_thing(),
                    false => *right,
                }
            Expression::Binary { operation,mut left, right }  => {
                *left = left.remove_leftmost_zero();
                let joined = Expression::Binary { operation , left , right };
                println!("new joined {joined}");

                joined
            }
            _ => self,
        };

        println!("removed zero result = {returned}");

        returned
    }

    /// Note : Used as `if let` guards are experimental at time of writing
    /// 
    /// See issue #51114 <https://github.com/rust-lang/rust/issues/51114> for more information
    fn is_removable(&self) -> bool {
        println!("is_removable called for {self}");
        match self {
            Expression::Term(ref term) => term.variables.is_empty() && term.coefficient == 0,
            _ => false,
        }
    }
    
    fn negate_leftmost_thing(self) -> Self {
        println!("negate_leftmost_thing called for {self}");
        println!("{self} became");
        let negagted = match self {
            Expression::Term(term) => (-term).into(),
            Expression::Function(func) => (-func).into(),
            Expression::Nested(inner) => (-*inner).into(),
            Expression::Binary { operation,mut left, right } => {
                *left = left.negate_leftmost_thing();
                Expression::Binary { operation, left, right }   
            }
        };
        println!("{negagted}");
        negagted
    }
}