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

    fn move_add(other : Self,right : Self,left : Self,variables_to_count : &Variables) -> (Self,Self) {
        Self::move_add_or_sub_inner(
            other, left, right, variables_to_count,
            |left,right| Expression::new_plus(left, right),
            |left,right| left - right,
            |expr,vec| expr.into_move_add_from(vec)
        )
    }

    fn move_sub(other : Self,right : Self,left : Self,variables_to_count : &Variables) -> (Self,Self) {
        Self::move_add_or_sub_inner(
            other, left, right, variables_to_count,
            |left,right| Expression::new_minus(left, right),
            |left,right| left + right,
            |expr,vec| expr.into_move_sub_from(vec)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Expression, parse_expression, parse_term};


    #[test]
    fn move_nested(){
        let(lexpr,rexpr) = Expression::move_nested(
            5.into()
            ,Expression::new_nested(parse_expression("2x + 4").unwrap().1.unwrap())
        );
        assert_eq!(&lexpr.to_string(),"0");
        assert_eq!(&rexpr.to_string(),"5 - (2x + 4)");
    }

    #[test]
    fn move_durch(){
        // (3y-6)/9 = 4x
        let(lexpr,rexpr) = Expression::move_durch(
            parse_term("4x").unwrap().1.into(),
            9.into(),
            parse_expression("3y - 6").unwrap().1.unwrap(),
        );
        assert_eq!(&lexpr.to_string(),"3y - 6");
        assert_eq!(&rexpr.to_string(),"36x");
    }

    #[test]
    fn move_mal() {
        // (2z + 3) * 6 = 5y
        let (lexpr, rexpr) = Expression::move_mal(
            parse_term("5y").unwrap().1.into(),
            6.into(),
            parse_expression("2z + 3").unwrap().1.unwrap(),
            &[('z',1f64.into())].into_iter().collect()
        );
        assert_eq!(&lexpr.to_string(), "2z + 3");
        assert_eq!(&rexpr.to_string(), "5y/6");
    }

    #[test]
    fn move_add() {
        // 2x - 5y + 4x = 3z + 2y - 6
        // -5y = 3z + 2y - 6 + 6x
        let (lexpr, rexpr) = Expression::move_add(
            parse_expression("3z + 2y - 6").unwrap().1.unwrap(),
            parse_expression("2x - 5y").unwrap().1.unwrap(),
            parse_expression("4x").unwrap().1.unwrap(),
            &[('y',1f64.into())].into_iter().collect()
        );
        assert_eq!(&rexpr.to_string(), "3z + 2y - 6 + 6x");
        assert_eq!(&lexpr.to_string(), "-5y");
    }
}

/*
impl Expression {
    fn move_add(other : Self,left : Self,right : Self,variables_to_count : &Variables) -> (Self,Self) {
        let is_left_mul_or_div = matches!(left,Expression::Binary { ref operation, .. } if operation == &ArithmeticOperation::Mal && operation == &ArithmeticOperation::Durch);
        let is_right_mul_or_div = matches!(right,Expression::Binary { ref operation, .. } if operation == &ArithmeticOperation::Mal && operation == &ArithmeticOperation::Durch);

        match (is_left_mul_or_div,is_right_mul_or_div) {
            // Both true so eg 2x(x) + (3/x) or smth like that
            // So decide which one to move 
            // For now use count_variable_occurrences
            (true, true) => {    
                // TODO : Find a better way then this to handle in future

                let lexpr_count = left.count_variable_occurrences(&variables_to_count);
                let rexpr_count = right.count_variable_occurrences(&variables_to_count);  
                
                match lexpr_count.cmp(&rexpr_count) {
                    Ordering::Less => (right,other - left),
                    Ordering::Greater => (left,other - right),
                    Ordering::Equal => (left,other - right)
                }
            },
            // So eg 2x(x) + 7y or smth like that
            // So move the false one 
            (true, false) => {
                let mut vec = Vec::new();

                match left.move_add_if_children_are_add_or_sub(&mut vec, variables_to_count) {
                    None => (right,other.from_move_into(vec)),
                    Some(new_left) => (new_left + right,other.from_move_into(vec)),
                }
            }
            // So eg 7y + 2x(x) or smth like that
            // So move the false one 
            (false, true) => {
                let mut vec = Vec::new();

                match right.move_add_if_children_are_add_or_sub(&mut vec, variables_to_count) {
                    None => (left,other.from_move_into(vec)),
                    Some(new_right) => (left + new_right,other.from_move_into(vec)),
                }
            }
            // So eg 7y + 2x or smth like that
            // So try to move both
            (false,false) => {
                let mut vec = Vec::new();

                let new_right = right.move_add_if_children_are_add_or_sub(&mut vec, variables_to_count);
                let new_left = left.move_add_if_children_are_add_or_sub(&mut vec, variables_to_count);

                match (new_left,new_right) {
                    (None, None) => (0.into(),other.from_move_into(vec)),
                    (None, Some(expression)) | (Some(expression), None) => (expression,other.from_move_into(vec)),
                    (Some(lexpr), Some(rexpr)) => (lexpr + rexpr,other.from_move_into(vec)),
                }
            }
        }
    }

    fn from_move_into(self,mut vec : Vec<Expression>) -> Self {
        match vec.pop() {
            // no terms to move
            None => self,
            Some(mut expr_to_subtract_from_other) => {     
                for expr in vec.into_iter(){
                    expr_to_subtract_from_other = Expression::new_minus(expr_to_subtract_from_other, expr)
                }

                self - expr_to_subtract_from_other
            },
        }
    }

    fn move_add_if_children_are_add_or_sub(self,vec :&mut Vec<Expression> /* Expression to avoid unneccessary conversion btw term and expressino */,variables_to_count : &Variables) -> Option<Self> {
        match self {
            Self::Term(ref term) if term.contains_any_variable(&mut variables_to_count.keys()) => Some(self),
            Self::Term(_) => {
                // + + 3 then + - 3 then expressino --3 is + 3 so nothing
                vec.push(self); 
                None // to tell tree has been 'removed' completely
            },
            Self::Nested(expr) => Self::move_add_if_children_are_add_or_sub(*expr, vec, variables_to_count),
            Self::Binary { ref operation, .. } 
                if operation == &ArithmeticOperation::Mal || operation == &ArithmeticOperation::Durch => Some(self),

            Self::Binary { operation, left,right } 
                if operation == ArithmeticOperation::Plus || operation == ArithmeticOperation::Minus => {
                let lhs = Self::move_add_if_children_are_add_or_sub(*left, vec, variables_to_count);
                let rhs = Self::move_add_if_children_are_add_or_sub(*right, vec, variables_to_count);
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
}*/