/// A utility struct for evaluating a target expression.
///
/// The `EvaluateNoValues` struct allows you to evaluate a mathematical expression.
/// It stores the expression to be evaluated and the values of variables used in the expression.
/// It is created by [super::Evaluate::evaluate]
#[derive(Debug, Clone)]
pub struct EvaluateNoValues<T>(pub(super) T);

/// A utility struct for evaluating a target expression with a single value substitution.
///
/// The `EvaluateWithSingleValue` struct allows you to evaluate a mathematical expression
/// with a single variable replacement. It stores the expression, the variable to replace, and the
/// new value of the variable.
/// It is created by [super::Evaluate::evaluate_with_single_value]
#[derive(Debug, Clone)]
pub struct EvaluateWithSingleValue<T, V>(pub(super) crate::manipulation::SingleVariableReplacements<T,V>);

/// A utility struct for evaluating a target expression with multiple variable replacements.
///
/// The `EvaluateWithMultipleValues` struct allows you to evaluate a mathematical expression
/// with multiple variable replacements. It stores the expression and a map of variable-value pairs.
/// It is created by [super::Evaluate::evaluate_with_multiple_values]
#[derive(Debug, Clone)]
pub struct EvaluateWithMultipleValues<'a,T, V>(pub(super) crate::manipulation::MultipleVariableReplacements<'a,T, V>);