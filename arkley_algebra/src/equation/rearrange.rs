use std::{cmp::Ordering, vec};

use num_notation::Signed;

use crate::{
    Expression, Term, Variables, ArithmeticOperation, 
    manipulation::VariableAnalysis
};

impl Expression {
    /// Rearrange the equation to isolate specific variables on one side.
    ///
    /// This function attempts to rearrange the equation so that specific variables, as
    /// specified by `variables_to_count`, is isolated on one side of the equation.
    ///
    /// # Arguments
    ///
    /// - `self`: A side of equation to rearrange.
    /// - `other`: Another side of equation to rearrange.
    /// - `variables_to_count`: A reference to the variables to isolate.
    ///
    /// # Returns
    ///
    /// New equations representing the rearranged equation where the specified variable
    /// is isolated on one side.
    pub(super) fn rearrange<F>(
        self,
        other : Self,
        variables_to_count : &Variables,
        should_continue_rearranging : F
    ) -> (Self,Self) 
    where F : Fn(&Self,&Self) -> bool {
        let (lexpr,rexpr) = match self {
            Self::Nested(inner) => Self::move_nested(other, *inner),

            Self::Binary { operation, left, right } 
                if operation == ArithmeticOperation::Durch => Self::move_durch(other, *right, *left),
            Self::Binary { operation, left, right } 
                if operation == ArithmeticOperation::Mal => Self::move_mal(other, *right, *left, variables_to_count),
            Self::Binary { operation, left, right } 
                if operation == ArithmeticOperation::Plus => Self::move_add(other, *right, *left, variables_to_count),
            Self::Binary { operation, left, right } 
                if operation == ArithmeticOperation::Minus => Self::move_sub(other, *right, *left, variables_to_count),

            // Do nothing if contains variables from new subject (for now)
            Self::Term(ref term) if term.contains_all(&mut variables_to_count.keys()) => (self,other),
            Self::Term(term) => Self::move_term(other, term),

            _ => unreachable!()
        };

        match should_continue_rearranging(&lexpr,&rexpr) {
            true => lexpr.rearrange(rexpr, variables_to_count,should_continue_rearranging),
            false => (lexpr,rexpr),
        }
    }
}

impl Expression {
    fn move_nested(other : Self,inner : Self) -> (Self,Self) {
        (0.into(),Expression::new_minus(other, inner)) 
    } 

    fn move_term(other : Self,term : Term) -> (Self,Self) {
        let operation = match term.coefficient.is_positive() {
            true => ArithmeticOperation::Minus,
            false => ArithmeticOperation::Plus
        };

        (0.into(),Expression::new_binary(operation,other,term.into()))
    }

    fn move_durch(other : Self,right : Self,left : Self) -> (Self,Self) {
        (left,other * right)
    } 

    fn move_mal(other : Self,right : Self,left : Self,variables_to_count : &Variables) -> (Self,Self) {
        // TODO : Find a better way then this to handle in future

        let lexpr_count = left.count_variable_occurrences(&variables_to_count);
        let rexpr_count = right.count_variable_occurrences(&variables_to_count);
        
        match lexpr_count.cmp(&rexpr_count) {
            Ordering::Less => (right,other / left),
            Ordering::Greater => (left,other / right),
            Ordering::Equal => (left * right,other)
        }
    } 

    fn move_add(other : Self,left : Self,right : Self,variables_to_count : &Variables) -> (Self,Self) {
        Self::move_add_or_sub_inner(
            other, left, right, variables_to_count,
            |left,right| Expression::new_plus(left, right),
            |left,right| left - right,
        )
    }

    fn move_sub(other : Self,left : Self,right : Self,variables_to_count : &Variables) -> (Self,Self) {
        Self::move_add_or_sub_inner(
            other, left, right, variables_to_count,
            |left,right| Expression::new_minus(left, right),
            |left,right| left + right,
        )
    }
}