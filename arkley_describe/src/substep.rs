
/// Represents a substep in the description of numeric operations.
#[derive(Debug)]
pub struct SubStep {
    /// Informational text for the substep.
    info: String,
    /// LaTeX representation of the substep (for mathematical notation).
    latex: Option<String>,
}

impl SubStep {
    /// Creates a new instance of `SubStep` with the given informational text
    pub const fn new(info: String) -> Self {
        Self { info, latex : None }
    }

    /// Creates a new instance of `SubStep` with the given informational text and latxes
    pub const fn new_with_latex(info: String,latex : Option<String>) -> Self {
        Self { info, latex }
    }

    /// Updates latex of current instance
    pub fn set_latex(&mut self,latex : String) {
        self.latex = Some(latex);
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
    pub fn latex(&self) -> &Option<String> {
        &self.latex
    }
}