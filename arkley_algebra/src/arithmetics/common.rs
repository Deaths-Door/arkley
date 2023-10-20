use std::collections::{BTreeMap, HashMap};
use num_notation::{
    Number,
    fraction::Signed,
};
use crate::{Expression,Term,Variables,ArithmeticOperation, Function, FunctionArguments};

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

impl Function {
    fn with_expr(mut self,expression : Option<Box<Expression>>) -> Self {
        self.expression = expression;
        self
    }
}
#[derive(Hash)]
struct FunctionArgumentsWithEquality {
    arguments : FunctionArguments,
    expression : Option<Box<Expression>>,
    closure : fn(Function) -> Expression,
}

impl FunctionArgumentsWithEquality {
    fn new(function : Function) -> Self { 
        let arguments = function.arguments;
        let expression = function.expression;
        let closure = function.closure;

        Self { arguments, expression, closure } 
    }
}


impl Eq for FunctionArgumentsWithEquality {}

impl PartialEq for FunctionArgumentsWithEquality {
    // If both instances are expressions (expr),
    // and if both expressions are terms (term),
    // then it tries to compare them for equality (eq).
    // If any of these conditions are not met, it returns false.
    fn eq(&self, other: &Self) -> bool {
        self.arguments.iter()
            .all(|(key,_svalue)| match other.arguments.get(key) {
                None => false,
                Some(_ovalue) => match (_svalue,_ovalue) {
                    (Some(svalue), Some(ovalue)) => match (svalue,ovalue) {
                        (Expression::Term(t1), Expression::Term(t2)) => match t1.variables.is_empty() && t2.variables.is_empty() {
                            true => t1.coefficient == t2.coefficient,
                            false => false
                        },
                        _ => false
                    },
                    _ => false
                }
            })
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
    fn collect_terms(self,term_map : &mut BTreeMap<Variables,Number>,fn_map : &mut HashMap<&'static str,HashMap<FunctionArgumentsWithEquality,i16>>) -> Option<Expression> {
        match self {
            Self::Nested(inner) => Some(Expression::new_nested(inner.combine_terms())),
            Self::Term(term) => {
                term_map.entry(term.variables)
                    .and_modify(|value| *value += term.coefficient.clone())
                    .or_insert(term.coefficient);
                None
            }
            Self::Function(func) => {
                match fn_map.get_mut(func.name) {
                    Some(hashmap) => {
                        hashmap.entry(FunctionArgumentsWithEquality::new(func))
                        .and_modify(|value| *value += 1 )
                        .or_insert(1);
                    },
                    None => {
                        let mut map = HashMap::new();

                        let name = func.name;

                        let key = FunctionArgumentsWithEquality::new(func);
                        map.insert(key,1);

                        fn_map.insert(name,map);

                    },
                }                                
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
                        match fn_map.get_mut(func.name) {
                            Some(hashmap) => {
                                hashmap.entry(FunctionArgumentsWithEquality::new(func))
                                .and_modify(|value| *value -= 1 )
                                .or_insert(-1);
                            },
                            None => {
                                let mut map = HashMap::new();
                                
                                let name = func.name;
                                
                                let key = FunctionArgumentsWithEquality::new(func);
                                map.insert(key,-1);
        
                                fn_map.insert(name, map);
                            },
                        }
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
        functions : HashMap<&'static str,HashMap<FunctionArgumentsWithEquality,i16>>,
        nested_expr : Option<Expression>
    ) -> Self {
        let mut expression : Expression = Term::new(Number::Decimal(0.0)).into();

        // sort by name then by count
        let mut sort_function_data : Vec<(&str, HashMap<FunctionArgumentsWithEquality, i16>)> = functions.into_iter().collect();
        sort_function_data.sort_by_key(|(key, _)| *key);

        for (name,hashmap) in sort_function_data {
            let mut sort_count_data :  Vec<(FunctionArgumentsWithEquality,i16)> = hashmap.into_iter().collect();
            sort_count_data.sort_by_key(|(_,value)| *value);

            for (_arguments, count) in sort_count_data {
                let func : Expression = Function::new(name,_arguments.closure)
                    .with_arguments(_arguments.arguments)
                    .with_expr(_arguments.expression)
                    .into();
                
                expression = match count.is_positive() {
                    true => Expression::new_plus(
                        expression, 
                        match count == 1 {
                            true => func,
                            false => Expression::new_mal(
                                count.into(), 
                                func
                            )
                        } 
                    ),
                    false => Expression::new_minus(
                        expression, 
                        Expression::new_mal(
                            (-count).into(), 
                            func
                        )
                    )
                }
            }
        }

        for (variables,coefficient) in terms {
            expression = match coefficient.is_positive() {
                true => Expression::new_plus(expression, Term::new_with_variable(coefficient,variables).into()),
                // If the coefficient is negative (-coefficient), the sign can be '-', but the number itself is positive. 
                // For example, -3 represents a negative number, whereas --3 is not equal to -3; it represents a positive number.
                false => Expression::new_minus(expression, Term::new_with_variable(-coefficient,variables).into()),
            }
        }

        if let Some(nested) = nested_expr {
            expression = Self::new_plus(expression,nested);
        }
        
        expression
    }

    /// Combines terms within the expression.
    ///
    /// # Returns
    ///
    /// The expression with combined terms.
    pub(in crate::arithmetics) fn combine_terms(self) -> Expression {
        let mut term_map = BTreeMap::new();
        
        // $name -> {[args] -> $count}
        // $count -> { $name -> [args] }
        let mut fn_map = HashMap::new();

        let nested_expr = self.collect_terms(&mut term_map,&mut fn_map);
    
        Self::reconstruct_expression(term_map,fn_map,nested_expr)
    }
}