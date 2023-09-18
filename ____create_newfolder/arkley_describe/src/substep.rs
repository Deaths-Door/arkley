/// Represents a substep in the description of an operation .
#[derive(Debug)]
pub struct SubStep {
    /// Description of the substep.
    description: String,

    /// LaTeX representation of the substep (for mathematical notation).
    latex: Option<String>,
}

impl SubStep {
    /// Creates new instance of substep with description
    pub const fn new(description: String)-> Self {
        Self { description , latex : None }
    }

    /// Set the LaTeX representation of the SubStep.
    pub fn set_latex(&mut self, latex: String) {
        self.latex = Some(latex);
    }

    /// Set the description of the SubStep.
    pub fn set_description(&mut self, latex: String) {
        self.latex = Some(latex);
    }

    /// Get the description of the SubStep.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the LaTeX representation of the SubStep (if available).
    pub const fn latex(&self) -> &Option<String> {
        &self.latex
    }
}