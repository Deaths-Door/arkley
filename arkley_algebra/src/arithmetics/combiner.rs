use std::{collections::{HashMap,BTreeMap},collections::btree_map::Entry as BtreeEntry,collections::hash_map::Entry as HashEntry, hash::{Hash, Hasher}};
use itertools::Itertools;
use num_notation::{fraction::Signed, Number};
use crate::{ArithmeticOperation, CustomizableExpression, Expression, Term, Variables};

type TermBtreeMap = BTreeMap<Variables,Number>;
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
    fn collect_terms(self,term_map : &mut TermBtreeMap,custom_map : &mut CustomHashMap) -> Option<Expression> {
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
        mut term_map: TermBtreeMap,
        custom_map : CustomHashMap,
        nested_expr : Option<Expression>
    ) -> Self {  
        let mut customs : Vec<_> = custom_map.into_iter().sorted_by_key(|(_,v)| *v).collect();

        let mut expression : Expression = match term_map.is_empty() {
            true => Self::from_custom(&mut customs),
            false => Self::from_terms(&mut term_map) 
        };

        expression = expression.join_terms(term_map.into_iter())
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
        let mut term_map = TermBtreeMap::new();
        let mut custom_map = CustomHashMap::new();
        let nested_expr = self.collect_terms(&mut term_map,&mut custom_map);
        Self::reconstruct_expression(term_map,custom_map,nested_expr)
    }
}

impl Expression {
    /// Subtracts a term from a term map.
    ///
    /// This is a convenience wrapper around `modify_term_entry`. It takes a term
    /// and modifies the corresponding entry in the term map by **subtracting** its
    /// coefficient (which is negated before insertion). This effectively removes
    /// the term from the polynomial or reduces its coefficient if it already exists.
    ///
    /// # Arguments
    ///
    /// * `term_map` - A mutable reference to the term map to modify.
    /// * `term` - The term to subtract.
    fn modify_add_term_entry(term_map : &mut TermBtreeMap,term : Term) {
        Self::modify_term_entry(term_map, term.variables,term.coefficient)
    }

    /// Adds a term to a term map.
    ///
    /// This is a convenience wrapper around `modify_term_entry`. It takes a term
    /// and modifies the corresponding entry in the term map by **adding** its
    /// coefficient. This effectively adds the term to the polynomial.
    ///
    /// # Arguments
    ///
    /// * `term_map` - A mutable reference to the term map to modify.
    /// * `term` - The term to add.
    fn modify_sub_term_entry(term_map : &mut TermBtreeMap,term : Term) {
        Self::modify_term_entry(term_map, term.variables,-term.coefficient)
    }

    /// Modifies the coefficient of a term in a term map.
    ///
    /// This function takes a mutable reference to a `TermBtreeMap`, the variables
    /// of the term to modify, and the new coefficient. It attempts to find the
    /// entry in the map corresponding to the given variables.
    ///
    /// If the entry is vacant (meaning no term with those variables exists),
    /// a new entry with the provided coefficient is inserted.
    ///
    /// If the entry is occupied (meaning a term with those variables already
    /// exists), the existing coefficient is replaced with the new coefficient.
    ///
    /// # Arguments
    ///
    /// * `term_map` - A mutable reference to the term map to modify.
    /// * `variables` - The variables of the term to modify.
    /// * `coefficient` - The new coefficient for the term.
    fn modify_term_entry(term_map : &mut TermBtreeMap,variables : Variables,coefficient : Number) {
        match term_map.entry(variables) {
            BtreeEntry::Vacant(entry) => {
                entry.insert(coefficient);
            },
            BtreeEntry::Occupied(mut entry) => {
                entry.insert(coefficient);
            }
        }
    }
}

impl Expression {
    // Adds a custom expression to a custom hash map, effectively treating it
    /// as a term with a coefficient of 1.
    ///
    /// This is a convenience wrapper around `modify_custom_entry`. It takes a
    /// boxed pointer to a `CustomizableExpression` and modifies the corresponding
    /// entry in the custom hash map by adding it with a coefficient of 1. This
    /// effectively adds the custom expression to the expression represented by
    /// `Expression`.
    ///
    /// # Arguments
    ///
    /// * `customs_map` - A mutable reference to the custom hash map to modify.
    /// * `custom` - A boxed pointer to the custom expression to add.
    fn modify_add_custom_entry(customs_map : &mut CustomHashMap,custom : Box<dyn CustomizableExpression>) {
        Self::modify_custom_entry(customs_map, custom,1)
    }

    /// Subtracts a custom expression from a custom hash map, effectively treating
    /// it as a term with a coefficient of -1.
    ///
    /// This is a convenience wrapper around `modify_custom_entry`. It takes a
    /// boxed pointer to a `CustomizableExpression` and modifies the corresponding
    /// entry in the custom hash map by adding it with a coefficient of -1. This
    /// effectively removes the custom expression from the expression represented by
    /// `Expression` or reduces its coefficient if it already exists.
    ///
    /// # Arguments
    ///
    /// * `customs_map` - A mutable reference to the custom hash map to modify.
    /// * `custom` - A boxed pointer to the custom expression to subtract.
    fn modify_sub_custom_entry(customs_map : &mut CustomHashMap,custom : Box<dyn CustomizableExpression>) {
        Self::modify_custom_entry(customs_map,custom,-1)
    }

    /// Modifies the coefficient of a custom expression in a custom hash map.
    ///
    /// If the entry is vacant (meaning no custom expression with the same
    /// underlying data exists), a new entry with the provided coefficient is
    /// inserted.
    ///
    /// If the entry is occupied (meaning a custom expression with the same
    /// underlying data already exists), the existing coefficient is replaced
    /// with the new coefficient.
    ///
    /// # Arguments
    ///
    /// * `customs_map` - A mutable reference to the custom hash map to modify.
    /// * `custom` - A boxed pointer to the custom expression to modify.
    /// * `coefficient` - The new coefficient for the custom expression.
    fn modify_custom_entry(customs_map : &mut CustomHashMap,custom : Box<dyn CustomizableExpression>,coefficient :i16) {
        match customs_map.entry(custom) {
            HashEntry::Vacant(entry) => {
                entry.insert(coefficient);
            },
            HashEntry::Occupied(mut entry) => {
                entry.insert(coefficient);
            }
        }
    }
}

// TODO;doc this
impl Expression {
    fn join_nested_expression(self,nested_expr : Option<Expression>) -> Self { 
        if let Some(nested) = nested_expr {
            return Self::new_plus(self,nested);
        }
        
        self
    }

    fn from_terms(terms : &mut TermBtreeMap) -> Self {
        let (variables,coefficient) = terms.pop_first().unwrap();
        Term::new_with_variables(coefficient,variables).into()
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

    fn join_terms(mut self,iterator : impl Iterator<Item =(Variables,Number)>) -> Self {
        for (variables,coefficient) in iterator {
            self = match coefficient.is_positive() {
                true => {
                    let term = Term::new_with_variables(coefficient.clone(),variables);
                    Expression::new_plus(self, term)
                }
                
                // If the coefficient is negative (-coefficient), the sign can be '-', but the number itself is positive. 
                // For example, -3 represents a negative number, whereas --3 is not equal to -3; it represents a positive number.
                false => {
                    let term = Term::new_with_variables(-coefficient,variables);
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