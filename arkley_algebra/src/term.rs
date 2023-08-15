use std::ops::{Add,Sub,Mul,Div};
use std::collections::{BTreeMap,BTreeSet};
use std::cmp::Ordering;
use arkley_numerics::Number;
use arkley_traits::Power;

use crate::Expression;

/// Represents a collection of variables, each associated with a numerical value.
/// The `Variables` type is an alias for `BTreeMap<char, Number>`.
pub type Variables = BTreeMap<char,Number>;

/// A struct representing a mathematical term.
///
/// A `Term` is a basic unit in a mathematical expression. It consists of a coefficient
/// (which can be any type that implements the `Numeric` trait) and variables represented
/// as `BTreeMap<char,Number>` .
#[derive(Debug,PartialEq,Clone)]
pub struct Term {
    /// The coefficient of the term.
    coefficient: Number,

    /// The variables and their exponents in the term.
    variables: Variables,
}
/*
impl Term {
    /// Creates new instance of Term using coefficient and variable
    pub const fn new_with_variable(coefficient: Number,variables: Variables) -> Self {
        Self { coefficient , variables }
    }

    /// Creates new instance of Term using coefficient
    pub const fn new(coefficient: Number) -> Self {
        Self { coefficient , variables : Variables::new() }
    }

    /// Tries to set the value of a variable in the `Term` object and updates the coefficient accordingly.
    ///
    /// # Parameters
    ///
    /// - `variable`: A reference to a `char` representing the variable whose value needs to be set.
    /// - `value`: A `Number` representing the new value for the variable.
    ///
    /// # Returns
    ///
    /// - `Some(())`: If the variable is found in the `Term`, it sets the value of the variable and updates the coefficient accordingly.
    /// - `None`: If the variable is not found in the `Term`, it returns `None`.
    ///
    pub fn try_set_variable_value(&mut self,variable : &char,value : Number) -> Option<()> {
        match self.variables.remove(variable) {
            None => None,
            Some(exponent) => {
                self.coefficient *= value.to_the_power_of(exponent);
                Some(())
            }
        }
    }
}


impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.coefficient)?;
        for (name,exponent) in self.variables.iter() {
            write!(f,"{name}")?;
            if exponent > &1 {
                write!(f,"^{exponent}")?;
            }
        }
        Ok(())
    }
}

/*
impl TryFrom<&str> for Term {
    /// temp return type
    type Error = ();
    fn try_from(value : &str) -> Result<Self, Self::Error> {        
        match value.find(|c: char| c.is_ascii_alphabetic()) {
            None => {
                let number = Number::try_from(value).map_err(|_| ())?;
                Ok(Term::new(number))
            },
            Some(v_index) => {
                let v_str = &value[v_index..];

                let mut variables = Variables::new();

                let is_char = |c :char| -> bool {
                    c >= 'a' && c <= 'z'
                };

                let mut v_iter = v_str.chars().peekable();
                loop {
                    match v_iter.next(){
                        None => break,
                        Some(ch) => match is_char(ch) {
                            false => return Err(()),
                            true => match v_iter.peek() {
                                None => break,
                                Some('^') => {
                                    let n : &str =  &v_iter
                                        .clone()
                                        .take_while(|c| is_char(*c))
                                        .map(|c| c.to_string())
                                        .collect::<Vec<_>>()
                                        .join("");

                                    let number = Number::try_from(n).map_err(|_| ())?;
                                    variables.insert(ch,number);
                                }
                                _ => return Err(())
                            }
                        }
                    }
                }
                /*let v_vec : Vec<_> = v_str.split_inclusive(|c:char| c >= 'a' && c <= 'z').collect();
            
                for item in v_vec {
                    let ch = item.chars().next().unwrap();
                    if item.len() == 1 {
                        variables.insert(ch,Number::Decimal(1.0));
                        continue;
                    }

                    if &item[0..1] != "^" {
                        return Err(());
                    }

                    let number = Number::try_from(&item[2..])?;
                    variables.insert(ch,number);
                }*/

                // to handle cases like -x or +xy
                let possible_sign_str = &value[..v_index];
                
                let term = if v_index == 1 || possible_sign_str.is_empty() ||possible_sign_str == "+" {
                    Term::new_with_variable(Number::Decimal(1.0), variables)
                }
                else if v_index == 1 || possible_sign_str == "-" {
                    Term::new_with_variable(Number::Decimal(-1.0), variables)
                } 
                else{
                    let n = Number::try_from(possible_sign_str).map_err(|_| ())?;
                    Term::new_with_variable(n,variables)
                };
                Ok(term)
            }
        }
    }
}*/

impl Add for Term {
    type Output = Expression;

    fn add(self,other : Term) -> Self::Output {
        if self.variables == other.variables {
            let coefficient = self.coefficient + other.coefficient;
            let variables = self.variables;
            return Expression::new_term(Term::new_with_variable(coefficient,variables));
        }
        Expression::new_plus(self.into(),other.into())
    }
}

impl Sub for Term {
    type Output = Expression;

    fn sub(self,other : Term) -> Self::Output {
        if self.variables == other.variables {
            let coefficient = self.coefficient - other.coefficient;
            let variables = self.variables;
            return Expression::new_term(Term::new_with_variable(coefficient,variables));
        }
        Expression::new_minus(self.into(),other.into())
    }
}

impl Mul for Term {
    type Output = Expression;

    fn mul(self,other : Term) -> Self::Output {
        let coefficient = self.coefficient * other.coefficient;
        let mut variables = self.variables;
        for (&var,&ref exponent) in &other.variables {
            *variables.entry(var).or_insert(Number::Decimal(0.0)) += exponent;//.clone();
        };

        Expression::new_term(Term::new_with_variable(coefficient,variables))
    }
}

impl Div for Term {
    type Output = Expression;

    fn div(self,other : Term) -> Self::Output {
        let s_keys: BTreeSet<_> = self.variables.keys().cloned().collect();
        let o_keys: BTreeSet<_> = other.variables.keys().cloned().collect();

        let common : BTreeSet<_> = s_keys.intersection(&o_keys).cloned().collect();
        let s_unique_keys: BTreeSet<_> = s_keys.difference(&common).collect();
        let o_unique_keys: BTreeSet<_> = o_keys.difference(&common).collect();

        let mut s_variables = Variables::new();
        let mut o_variables =Variables::new();

        for key in s_unique_keys {
            s_variables.insert(*key,self.variables[key].clone());
        }

        for key in o_unique_keys {
            o_variables.insert(*key,other.variables[key].clone());
        }

        for key in common {
            let s_exponent = self.variables[&key].clone();
            let o_exponent = other.variables[&key].clone();

            let result = s_exponent - o_exponent;

            match result.partial_cmp(&0).unwrap() {
                // if equal so x^2 - x^2 then just ignore it
                Ordering::Equal => {}
                // if top > bottom so x^5 - x^3 = x^2 so top.variable(key).power = result
                Ordering::Greater => {
                    s_variables.insert(key,result);
                },
                // if top M bottom so x^2 - x^5 = x^-3 so bottom.variable(key).power = result
                Ordering::Less => {
                    o_variables.insert(key,result);
                },
            };
        }

        match s_variables.is_empty() && o_variables.is_empty() {
            true => Expression::new_term(Term::new(self.coefficient / other.coefficient)),
            false => {
                let t1 = Term::new_with_variable(self.coefficient,s_variables);
                // so if x / 1 is equal to 1 so simplify it
                match other.coefficient == 1 && o_variables.is_empty() {
                    true => Expression::new_term(t1),
                    false => {
                        let t2 = Term::new_with_variable(other.coefficient,o_variables);
                        Expression::new_durch(t1.into(),t2.into())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_terms_with_same_variables() {
        // 2.5x
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));
        //3.5x
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(1.0))]));

        // 2.5x + 3.5x = 6x
        let result = term1.clone() + term2.clone();

        // 6x
        let expected_term = Term::new_with_variable(Number::Decimal(6.0), Variables::from([('x', Number::Decimal(1.0))]));
        
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn add_terms_with_different_variables() {
        //2.5x
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));

        //3.5y
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('y', Number::Decimal(1.0))]));

        // 2.5x + 3.5y
        let result = term1.clone() + term2.clone();

        // 2.5x + 3.5y
        let expected_expression = Expression::new_plus(term1.into(), term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn add_terms_with_same_variables_and_different_powers() {
        // 2.5x^2
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 3.5x^3
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2 + 3.5x^2
        let result = term1.clone() + term2.clone();

        // 2.5x^2 + 3.5x^2
        let expected_expression = Expression::new_plus(term1.into(),term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn subtract_terms_with_same_variables() {
        // 5x
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(1.0))]));

        // 2.5x
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));

        // 5x - 2.5x
        let result = term1.clone() - term2.clone();


        // 2.5x
        let expected_term = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn subtract_terms_with_different_variables() {

        // 5x
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(1.0))]));

        // 5y
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('y', Number::Decimal(1.0))]));

        // 5x - 5y
        let result = term1.clone() - term2.clone();

        // 5x - 5y
        let expected_expression = Expression::new_minus(term1.into(), term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn subtract_terms_with_same_variables_and_different_powers() {
        // 5x^3
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 5x^3 - 2.5x^2
        let result = term1.clone() - term2.clone();

        let expected_expression = Expression::new_minus(term1.into(),term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn multiply_terms() {
        // 2x
        let term1 = Term::new_with_variable(Number::Decimal(2.0), Variables::from([('x', Number::Decimal(1.0))]));

        //3x^2
        let term2 = Term::new_with_variable(Number::Decimal(3.0), Variables::from([('x', Number::Decimal(2.0))]));

        // 2x * 3x^2
        let result = term1.clone() * term2.clone();

        // 6x^3
        let expected_term = Term::new_with_variable(Number::Decimal(6.0), Variables::from([('x', Number::Decimal(3.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn multiply_terms_with_same_variables_and_different_powers() {
        // 2.5x^2
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 3.5x^3
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2 * 3.5x^3
        let result = term1.clone() * term2.clone();

        // 2.5x^2 * 3.5x^3 = 8.75x^5
        let expected_term = Term::new_with_variable(Number::Decimal(8.75), Variables::from([('x', Number::Decimal(5.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    // Helper function to create a Term with a single variable.
     fn create_term_with_variable(coeff: i32, var: char, exp: i32) -> Term {
        let mut variables = Variables::new();
        variables.insert(var,Number::Decimal(exp as f64));
        Term::new_with_variable(Number::Decimal(coeff as f64), variables)
    }

    #[test]
    fn division_with_single_variable() {
        // Test division with a single variable (x^2 / x).
        let term1 = create_term_with_variable(1, 'x', 2);
        let term2 = create_term_with_variable(1, 'x', 1);
        // (x^2 / x)
        let result = term1 / term2;
    
        let expected = Expression::new_term(create_term_with_variable(1, 'x', 1));
        assert_eq!(result, expected);
    }

    #[test]
    fn division_with_constants() {
        // Test division with constants (6 / 3).
        let term1 = Term::new(Number::Decimal(6.0));
        let term2 = Term::new(Number::Decimal(3.0));
        let result = term1 / term2;
        let expected = Expression::new_term(Term::new(Number::Decimal(2.0)));
        assert_eq!(result, expected);
    }

    #[test]
    fn division_with_common_variables() {
        // Test division with common variables (x^3 / x^2).
        let term1 = create_term_with_variable(1, 'x', 3);
        let term2 = create_term_with_variable(1, 'x', 2);
        let result = term1 / term2;
        let expected = Expression::new_term(create_term_with_variable(1, 'x', 1));
        assert_eq!(result, expected);
    }

    #[test]
    fn division_with_unique_variables() {
        let mut variables1 = Variables::new();
        variables1.insert('x', Number::Decimal(3.0));
        variables1.insert('y', Number::Decimal(2.0));

        // x^3 * y^2
        let term1 = Term::new_with_variable(Number::Decimal(1.0), variables1);

        let mut variables2 = Variables::new();
        variables2.insert('x',Number::Decimal(2.0));
        variables2.insert('z',Number::Decimal(1.0));
        // x^2 * z.
        let term2 = Term::new_with_variable(Number::Decimal(1.0), variables2);

        // (x^3 * y^2) / (x^2 * z).
        let result = term1 / term2;


        let ev1 = Variables::from([
            ('x',Number::Decimal(1.0)),
            ('y',Number::Decimal(2.0))
        ]);

        let et1 = Term::new_with_variable(Number::Decimal(1.0),ev1);
        
        let ev2 = Variables::from([
                ('z',Number::Decimal(1.0))
        ]);

        let et2 = Term::new_with_variable(Number::Decimal(1.0),ev2);
        // (x * y^2) / z
        let expected = Expression::new_durch(
            et1.into(),
            et2.into()
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn display_term() {
        let variables: Variables = [('x',Number::Decimal(2.0)), ('y', Number::Decimal(3.0))].iter().cloned().collect();
        let term = Term::new_with_variable(Number::Decimal(2.5),variables);
        assert_eq!(term.to_string(), "2.5x^2y^3");
        println!("{}",term.to_string());
    }

    #[test]
    fn display_term_single_variable() {
        let variables: Variables = [('x', Number::Decimal(1.0))].iter().cloned().collect();
        let term = Term::new_with_variable(Number::Decimal(3.0),variables);
        assert_eq!(term.to_string(), "3x");
    }

    #[test]
    fn display_term_constant() {
        let variables: Variables = Variables::new();
        let term = Term::new_with_variable(Number::Decimal(5.0),variables);

        assert_eq!(term.to_string(), "5");
    }

    #[test]
    fn try_set_variable_value_existing_variable() {
        // Create a term with the variable 'x' and exponent 2
        let mut term = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(2.0))]));

        // Set the value of variable 'x' to 7
        assert_eq!(term.try_set_variable_value(&'x', Number::Decimal(7.0)), Some(()));

        // Check that the coefficient is updated correctly
        assert_eq!(term.coefficient, Number::Decimal(5.0 * (7_i32.pow(2)) as f64));
    }

    #[test]
    fn try_set_variable_value_non_existing_variable() {
        // Create a term with the variable 'y' and exponent 3
        let mut term = Term::new_with_variable(Number::Decimal(3.0), Variables::from([('y', Number::Decimal(3.0))]));

        // Try to set the value of variable 'x' (non-existing variable) to 5
        assert_eq!(term.try_set_variable_value(&'x', Number::Decimal(5.0)), None);

        // Check that the coefficient remains unchanged
        assert_eq!(term.coefficient, Number::Decimal(3.0));
    }


    /*#[test]
    fn try_from_term_with_valid_input_no_variables() {
        let input = "123.45";
        let result = Term::try_from(input).unwrap();
        let expected = Term::new(Number::Decimal(123.45));
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_term_with_variables() {
        let input = "2x^3y^2z";
        let result = Term::try_from(input).unwrap();//.unwrap();
        
        let expected_variables: Variables = [('x', Number::Decimal(3.0)),
                                            ('y', Number::Decimal(2.0)),
                                            ('z', Number::Decimal(1.0))].iter().cloned().collect();

        let expected = Term::new_with_variable(Number::Decimal(2.0), expected_variables);
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_term_with_valid_input_no_exponents() {
        let input = "xyz";
        let result = Term::try_from(input);//.unwrap();
        assert!(result.is_ok());

        let ev = Variables::from(
            [
                ('x',Number::Decimal(1.0)),
                ('y',Number::Decimal(1.0)),
                ('z',Number::Decimal(1.0))
            ]
        );
        let expected = Term::new_with_variable(Number::Decimal(1.0), ev);
        assert_eq!(result.unwrap(), expected);
    }*/
}*/