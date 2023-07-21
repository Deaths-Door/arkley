
/// Represents a substep in the description of numeric operations.
pub struct SubStep {
    /// Informational text for the substep.
    pub info: String,
    /// LaTeX representation of the substep (for mathematical notation).
    pub latex: String,
}

impl SubStep {
    /// Creates a new instance of `SubStep` with the given informational text and LaTeX representation.
    pub const fn new(info: String, latex: String) -> Self {
        Self { info, latex }
    }
}