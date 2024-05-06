use std::{collections::HashMap, hash::{Hash, Hasher}};
use itertools::Itertools;
use num_notation::{fraction::Signed, Number};
use crate::{ArithmeticOperation, CustomizableExpression, Expression, Term, Variables};


#[derive(PartialEq, Eq)]
struct HashableVariables(Variables);

impl<T> From<T> for HashableVariables where Variables : From<T> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Hash for HashableVariables {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hashable_vec = self.0.iter().sorted_by_key(|(k,_)| *k).collect_vec();
        Hash::hash(&hashable_vec, state)
    }
}

type VariablesHashMap = HashMap<HashableVariables,Number>;
type CustomHashMap = HashMap<Box<dyn CustomizableExpression>,i16>;

impl Term {
    pub(in crate::arithmetics) fn is_combinable_with(&self,other : &Self) -> bool {
        self.variables == other.variables
    }

    pub(in crate::arithmetics) fn force_add_terms(self,other : Term) -> Self {
        let coefficient = self.coefficient + other.coefficient;
        let variables = self.variables;
        Term::new_with_variables(coefficient,variables)
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
    fn collect_terms(self,term_map : &mut VariablesHashMap,custom_map : &mut CustomHashMap) -> Option<Expression> {
        match self {
            Self::Term(term) => {
                term_map.entry(term.variables.into())
                    .and_modify(|value| *value += term.coefficient.clone())
                    .or_insert(term.coefficient);
                None
            },
            Self::Custom(value) => {
                custom_map.entry(value) 
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                None
            },
            Self::Binary { operation , left , right} if operation == ArithmeticOperation::Plus => {
                let lexpr = left.collect_terms(term_map,custom_map);
                let rexpr = right.collect_terms(term_map,custom_map);

                match (lexpr,rexpr) {
                    (None,None) => None,
                    (Some(expr),None) | (None,Some(expr)) => Some(expr),
                    (Some(expr1),Some(expr2)) => Some(Expression::new_binary(operation,expr1,expr2))
                }   
            },

            Self::Binary { operation , left , right} if operation == ArithmeticOperation::Minus => {
                let lexpr = left.collect_terms(term_map,custom_map);
                let rexpr = match *right {
                    Self::Term(term) => {
                        term_map.entry(term.variables.into())
                            .and_modify(|value| *value -= term.coefficient.clone()) // as +- equals -
                            .or_insert(-term.coefficient); // as operations is - so -number
                        None
                    },
                    _ => right.collect_terms(term_map,custom_map),
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
            }
        }
    }

    /// Reconstructs the expression based on grouped terms and an optional nested expression.
    ///
    /// # Returns
    ///
    /// The reconstructed expression.
    fn reconstruct_expression(
        term_map: VariablesHashMap,
        custom_map : CustomHashMap,
        nested_expr : Option<Expression>
    ) -> Self {  
        let mut terms : Vec<_> = term_map.into_iter()
            // Taken from the PartialOrd Implementation of BtreeMap (https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#impl-PartialOrd-for-BTreeMap%3CK,+V,+A%3E)
            .sorted_by(|(k1,_),(k2,_)| k1.0.iter().cmp(k2.0.iter()))
            .collect();

        let mut customs : Vec<_> = custom_map.into_iter().sorted_by_key(|(_,v)| *v).collect();

        let mut expression : Expression = match terms.is_empty() {
            true => Self::from_custom(&mut customs),
            false => Self::from_terms(&mut terms) 
        };

        expression = expression.join_terms(terms.into_iter())
            .join_customs(customs.into_iter())
            .join_nested_expression(nested_expr);

        expression
    }

    /// Combines terms within the expression.
    ///
    /// # Returns
    ///
    /// The expression with combined terms.
    pub(in crate::arithmetics) fn combine_terms(self) -> Expression {
        let mut term_map = VariablesHashMap::new();
        let mut custom_map = CustomHashMap::new();
        let nested_expr = self.collect_terms(&mut term_map,&mut custom_map);
        Self::reconstruct_expression(term_map,custom_map,nested_expr)
    }
}

impl Expression {
    fn join_nested_expression(self,nested_expr : Option<Expression>) -> Self { 
        if let Some(nested) = nested_expr {
            return Self::new_plus(self,nested);
        }
        
        self
    }

    fn from_terms(terms : &mut Vec<(HashableVariables,Number)>) -> Self {
        let (variables,coefficient) = terms.remove(0);
        Term::new_with_variables(coefficient,variables.0).into()
    }

    fn from_custom(customs : &mut Vec<(Box<dyn CustomizableExpression>,i16)>) -> Self {
        let (custom,count) = customs.pop().unwrap();
        let customs = Expression::Custom(custom);
        
        match count {
            1 => customs,
            _ => Expression::new_mal(
                count, 
                customs
            )
        }
    }

    fn join_terms(mut self,iterator : impl Iterator<Item =(HashableVariables,Number)>) -> Self {
        for (variables,coefficient) in iterator {
            self = match coefficient.is_positive() {
                true => {
                    let term = Term::new_with_variables(coefficient.clone(),variables.0);
                    Expression::new_plus(self, term)
                }
                
                // If the coefficient is negative (-coefficient), the sign can be '-', but the number itself is positive. 
                // For example, -3 represents a negative number, whereas --3 is not equal to -3; it represents a positive number.
                false => {
                    let term = Term::new_with_variables(-coefficient,variables.0);
                    Expression::new_minus(self, term)
                }
            }
        }

        self
    }

    fn join_customs(mut self,iterator : impl Iterator<Item =(Box<dyn CustomizableExpression>,i16)>) -> Self {
        for (custom,count) in iterator {
            let custom : Expression = Self::Custom(custom);

            if count == 1 {
                self = Expression::new_plus(self, custom);
                continue;
            }

            self = match count.is_positive() {
                true => Expression::new_plus(
                    self, 
                    Expression::new_mal(
                        count, 
                        custom
                    )
                ),
                false => Expression::new_minus(
                    self, 
                    Expression::new_mal(
                        -count, 
                        custom
                    )
                )
            }
        }

        self
    }
}