use std::cmp::Ordering;

use num_notation::Signed;

use crate::{Term, Variables, Expression, manipulation::VariableAnalysis, ArithmeticOperation};

// count_variable_occurrences

impl Term {
    /// Count the occurrences of variables in the term.
    ///
    /// This method counts the occurrences of variables in the term.
    ///
    /// # Parameters
    ///
    /// - `variables_to_count`: A reference to a set of variables to count occurrences for.
    ///
    /// # Returns
    ///
    /// Returns the count of variable occurrences in the term.
    fn count_variable_occurrences(&self, variables_to_count: &Variables) -> usize {
        self.variables
            .keys()
            .filter(|key| variables_to_count.contains_key(key))
            .count()
    }
}

impl Expression {
    /// Count the occurrences of variables in the expression.
    ///
    /// This method counts the occurrences of variables in the expression and its nested sub-expressions.
    ///
    /// # Parameters
    ///
    /// - `variables_to_count`: A reference to a set of variables to count occurrences for.
    ///
    /// # Returns
    ///
    /// Returns the count of variable occurrences in the expression.
    pub(super) fn count_variable_occurrences(&self, variables_to_count: &Variables) -> usize {
        match self {
            Expression::Term(term) => term.count_variable_occurrences(variables_to_count),
            Expression::Binary { left, right, .. } 
                => left.count_variable_occurrences(variables_to_count) + right.count_variable_occurrences(variables_to_count),
            Expression::Nested(inner) => inner.count_variable_occurrences(variables_to_count),
        }
    }
}

// ---------------

// into_move_from

impl Expression {
    fn reverse_tree(&mut self) {
        match self {
            Expression::Binary { operation, left, right } 
                if operation == &ArithmeticOperation::Plus => {
                left.reverse_tree();
                right.reverse_tree();
                std::mem::swap(left, right);
            },
            Expression::Binary { operation, left, right } 
                if operation == &ArithmeticOperation::Minus => {
                left; right;
                // TODO : Implement this 'reverse' fn for minus 
                println!("TODO : Implement this 'reverse' fn for minus ");
           //    left.reverse_tree();
               // right.reverse_tree();
          //      std::mem::swap(left, right);
            },
            Expression::Nested(nested) => nested.reverse_tree(),
            _ => ()
        }
    }
    
    pub(super) fn move_from(self,mut vec : Vec<Term>) -> Self {
        match vec.pop() {
            // no terms to move
            None => self,
            Some(mut __inital_term) => {
                let mut expression : Expression = __inital_term.into();

                for term in vec.into_iter() {
                    expression = match term.coefficient.is_positive() {
                        true => Expression::new_plus(expression, term.into()),
                        false => Expression::new_minus(expression, (-term).into()),
                    };
                }

                expression = self + expression;
                expression.reverse_tree();

                expression
            }
        }
    }
}

// ---------------

// collect_all_add_sub_term_till_mul_div

impl Expression {
    fn collect_all_add_sub_term_till_mul_div(self,vec :&mut Vec<Term>,variables_to_count : &Variables) -> Option<Self> {
        match self {
            Self::Term(ref term) if term.contains_any_variable(&mut variables_to_count.keys()) => Some(self),
            Self::Term(term) => {
                // + + 3 then + - 3 then expressino --3 is + 3 so nothing
                vec.push(term); 
                None // to tell tree has been 'removed' completely
            },
            Self::Nested(expr) => Self::collect_all_add_sub_term_till_mul_div(*expr, vec, variables_to_count),
            Self::Binary { ref operation, .. } 
                if operation == &ArithmeticOperation::Mal || operation == &ArithmeticOperation::Durch => Some(self),
            Self::Binary { operation, left,right } if operation == ArithmeticOperation::Plus => {
                let lhs = Self::collect_all_add_sub_term_till_mul_div(*left, vec, variables_to_count);
                let rhs = Self::collect_all_add_sub_term_till_mul_div(*right, vec, variables_to_count);
                match (lhs,rhs) {
                    (None,None) => None,
                    (Some(lexpr),Some(rexpr)) => {
                        Expression::new_binary(operation, lexpr, rexpr).into()
                    }
                    // if one tree has been removed we can just remove the binary op cuz 0 + .. = .. 
                    (Some(value),None) | (None,Some(value)) => Some(value)
                }
            },

            Self::Binary { operation, left,right } 
                if operation == ArithmeticOperation::Minus => {

                let lhs = Self::collect_all_add_sub_term_till_mul_div(*left, vec, variables_to_count);
                let rhs = Self::collect_all_add_sub_term_till_mul_div(-*right, vec, variables_to_count);
                match (lhs,rhs) {
                    (None,None) => None,
                    (Some(lexpr),Some(rexpr)) => {
                        Expression::new_binary(operation, lexpr, rexpr).into()
                    }
                    // if one tree has been removed we can just remove the binary op cuz 0 + .. = .. 
                    (Some(value),None) | (None,Some(value)) => Some(value)
                }
            },
            Self::Binary { .. } => unreachable!()
        }
    }
}

// ---------------

// move_add_or_sub_inner

impl Expression {
    pub(super) fn move_add_or_sub_inner<F1,F2>(
        other : Self,
        left : Self,
        right : Self,
        variables_to_count : &Variables,
        create_self : F1,
        inverse_operation : F2,
    ) -> (Self,Self) where 
    F1 :  Fn(Self,Self) -> Self ,
    F2 :  FnOnce(Self,Self) -> Self {
        let is_left_mul_or_div = matches!(left,Self::Binary { ref operation, .. } if operation == &ArithmeticOperation::Mal && operation == &ArithmeticOperation::Durch);
        let is_right_mul_or_div = matches!(right,Self::Binary { ref operation, .. } if operation == &ArithmeticOperation::Mal && operation == &ArithmeticOperation::Durch);

        match (is_left_mul_or_div,is_right_mul_or_div) {
            // Both true so eg 2x(x) + (3/x) or smth like that
            // So decide which one to move 
            // For now use count_variable_occurrences
            (true, true) => {    
                // TODO : Find a better way then this to handle in future

                let lexpr_count = left.count_variable_occurrences(&variables_to_count);
                let rexpr_count = right.count_variable_occurrences(&variables_to_count);  
                
                match lexpr_count.cmp(&rexpr_count) {
                    Ordering::Less => (right,inverse_operation(other,left)),
                    Ordering::Greater => (left,inverse_operation(other,right)),
                    Ordering::Equal => (left,inverse_operation(other,right))
                }
            },
            // So eg 2x(x) + 7y or smth like that
            // So move the false one 
            (true, false) => {
                let mut vec = Vec::new();

                match left.collect_all_add_sub_term_till_mul_div(&mut vec, variables_to_count) {
                    None => (right,other.move_from(vec)), 
                    Some(new_left) => (create_self(new_left,right),other.move_from(vec)),
                }
            }
            // So eg 7y + 2x(x) or smth like that
            // So move the false one 
            (false, true) => {
                let mut vec = Vec::new();

                match right.collect_all_add_sub_term_till_mul_div(&mut vec, variables_to_count) {
                    None => (left,other.move_from(vec)),
                    Some(new_right) => (create_self(left,new_right),other.move_from(vec)),
                }
            }
            // So eg 7y + 2x or smth like that
            // So try to move both
            (false,false) => {
                let mut vec = Vec::new();

                let new_right = right.collect_all_add_sub_term_till_mul_div(&mut vec, variables_to_count);
                let new_left = left.collect_all_add_sub_term_till_mul_div(&mut vec, variables_to_count);

                match (new_left,new_right) {
                    (None, None) => (0.into(),other.move_from(vec)),
                    (None, Some(expression)) | (Some(expression), None) => (expression,other.move_from(vec)),
                    (Some(lexpr), Some(rexpr)) => (create_self(lexpr,rexpr),other.move_from(vec)),
                }
            }
        }
    }
}

// ---------------
