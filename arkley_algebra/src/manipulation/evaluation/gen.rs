use crate::{
    Term, Expression, Function, ArithmeticOperation, 
    manipulation::{
        VariableSubstitution, 
        Find, 
        SingleVariableReplacements, 
        MultipleVariableReplacements
    }
};

use super::{
    Evaluate, EvaluateNoValues, 
    EvaluateWithSingleValue, EvaluateWithMultipleValues, EvaluteWithValues
};

impl Evaluate for Term {}
impl Evaluate for Expression {}
impl Evaluate for Function {}

#[cfg(feature="equation")]
impl Evaluate for crate::Equation {}

impl<T> EvaluteWithValues<T> for Term where Self : VariableSubstitution<T> {}
impl<T> EvaluteWithValues<T> for Expression where Self : VariableSubstitution<T> {}
impl<T> EvaluteWithValues<T> for Function where Self : VariableSubstitution<T> {}

#[cfg(feature="equation")]
impl<T> EvaluteWithValues<T> for crate::Equation where Self : VariableSubstitution<T> {}

impl Find for EvaluateNoValues<Term> {
    type Output = Term;
    fn find(self) -> Self::Output {
        self.0
    }
}

impl Find for EvaluateNoValues<Function> {
    type Output = Expression;

    fn find(self) -> Self::Output {
        (self.0.closure)(self.0)
    }
}

impl ArithmeticOperation {
    fn operate_on(&self,left : Expression,right : Expression) -> Expression {
        match self {
            Self::Plus => left + right,
            Self::Minus => left - right,
            Self::Mal => left * right,
            Self::Durch => left / right,
        }
    }
}

impl Find for EvaluateNoValues<Expression> {
    type Output = Expression;
    fn find(self) -> Self::Output {
        match self.0 {
            Expression::Term(_) => self.0,
            Expression::Binary { operation, left, right } => 
                operation.operate_on(left.evaluate().find(), right.evaluate().find()),
            Expression::Nested(inner) => inner.evaluate().find(),
            Expression::Function(func) => func.evaluate().find()
        }   
    }
}

#[cfg(feature="equation")]
impl Find for EvaluateNoValues<crate::Equation> {
    type Output = crate::Equation;
    fn find(self) -> Self::Output {
        let mut eq = self.0;
        eq.left = eq.left.evaluate().find();
        eq.right = eq.right.evaluate().find();
        eq
    }
}

impl<T,V> Find for EvaluateWithSingleValue<T,V> 
    where SingleVariableReplacements<T,V> : Find , 
    <SingleVariableReplacements<T, V> as Find>::Output : Evaluate,
    EvaluateNoValues<<SingleVariableReplacements<T, V> as Find>::Output> : Find
{
    type Output = <EvaluateNoValues<<SingleVariableReplacements<T, V> as Find>::Output> as Find>::Output;
    fn find(self) -> Self::Output {
        self.0.find().evaluate().find()
    }
}

impl<'a,T,V> Find for EvaluateWithMultipleValues<'a,T,V> 
    where MultipleVariableReplacements<'a,T,V> : Find , 
    <MultipleVariableReplacements<'a,T, V> as Find>::Output : Evaluate,
    EvaluateNoValues<<MultipleVariableReplacements<'a,T, V> as Find>::Output> : Find
{
    type Output = <EvaluateNoValues<<MultipleVariableReplacements<'a,T, V> as Find>::Output> as Find>::Output;
    fn find(self) -> Self::Output {
        self.0.find().evaluate().find()
    }
}
