
/// This trait defines a way to get a label representing an algebraic term.
pub trait Label<'a> {
    /// Returns a string slice representing the [label](https://tex.stackexchange.com/a/305700/302157) for the term.
    /// 
    /// ## Examples
    /// - $`x_{1}`$
    /// - $`e_{force}`$
    /// - $`sin\theta_{0}`$
    fn label(&'a self) -> Option<&'a str>;
}