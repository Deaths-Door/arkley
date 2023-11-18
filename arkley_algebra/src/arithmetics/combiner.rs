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

/// TODO : Make this so it 'handles' cases like a/a or things like that so it simplifys itself make it handle ^ or root and * in same manner even without evalute
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
                    // TODO : Change sign here maybe issue
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
        mut terms : BTreeMap<Variables,Number>,
        functions : HashMap<Function,i16>,
        nested_expr : Option<Expression>
    ) -> Self {  
        let mut functions_sorted_by_count : Vec<(Function,i16)> = functions.into_iter().collect();
        functions_sorted_by_count.sort_by_key(|(_,key)| *key);

        let mut expression : Expression = match functions_sorted_by_count.is_empty() {
            true => Self::from_terms(&mut terms),
            false => Self::from_function(&mut functions_sorted_by_count)
        };

        expression = expression.join_functions(functions_sorted_by_count)
            .join_terms(terms)
            .join_nested_expression(nested_expr);

        expression
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
    fn join_nested_expression(self,nested_expr : Option<Expression>) -> Self { 
        if let Some(nested) = nested_expr {
            return Self::new_plus(self,nested);
        }
        
        self
    }

    fn from_terms(terms : &mut BTreeMap<Variables,Number>) -> Self {
        let (variables,coefficient) = terms.pop_first().unwrap();
        Term::new_with_variable(coefficient,variables).into()
    }

    fn join_terms(mut self,terms : BTreeMap<Variables,Number>) -> Self {
        let _before = self.clone();

        for (variables,coefficient) in terms {
            self = match coefficient.is_positive() {
                true => {
                    let term = Term::new_with_variable(coefficient.clone(),variables);
                    Expression::new_plus(self, term)
                }
                
                // If the coefficient is negative (-coefficient), the sign can be '-', but the number itself is positive. 
                // For example, -3 represents a negative number, whereas --3 is not equal to -3; it represents a positive number.
                false => {
                    let term = Term::new_with_variable(-coefficient,variables);
                    Expression::new_minus(self, term)
                }
            }
        }

        self
    }

    fn from_function(functions : &mut Vec<(Function,i16)>) -> Self {
        let (_function,count) = functions.pop().unwrap();
        let function : Expression = _function.into();
        
        if count == 1 {
            return function;
        }

        Expression::new_mal(
            count, 
            function
        )
    }

    fn join_functions(mut self,functions : Vec<(Function,i16)>) -> Self {
        for (_function,count) in functions {
            let function : Expression = _function.into();
            if count == 1 {
                self = Expression::new_plus(self, function);
                continue;
            }

            self = match count.is_positive() {
                true => Expression::new_plus(
                    self, 
                    Expression::new_mal(
                        count, 
                        function
                    )
                ),
                false => Expression::new_minus(
                    self, 
                    Expression::new_mal(
                    -count, 
                        function
                    )
                )
            }
        }

        self
    }
}