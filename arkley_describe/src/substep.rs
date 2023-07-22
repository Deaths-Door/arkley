
/// Represents a substep in the description of numeric operations.
#[derive(Debug)]
pub struct SubStep {
    /// Informational text for the substep.
    info: String,
    /// LaTeX representation of the substep (for mathematical notation).
    latex: String,
}

impl SubStep {
    /// Creates a new instance of `SubStep` with the given informational text and LaTeX representation.
    pub const fn new(info: String, latex: String) -> Self {
        Self { info, latex }
    }

    /// Returns a reference to the informational text of the substep.
    ///
    /// # Returns
    ///
    /// A reference to the informational text of the substep.
    pub fn information(&self) -> &str {
        &self.info
    }

    /// Returns a reference to the LaTeX representation of the substep.
    ///
    /// # Returns
    ///
    /// A reference to the LaTeX representation of the substep.
    pub fn latex(&self) -> &str {
        &self.latex
    }
}