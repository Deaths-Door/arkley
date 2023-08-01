/// Represents a substep in the description of an operation .
#[derive(Debug)]
pub struct SubStep {
    /// Description of the substep.
    description: String,

    /// LaTeX representation of the substep (for mathematical notation).
    latex: Option<String>,

    /// Path to the diagram image (if any) representing infomation used in the substep.
    diagram_path: Option<String>,
}

impl SubStep {
    pub const fn new(description: String)-> Self {
        Self { description , latex : None , diagram_path : None }
    }

    /// Set the LaTeX representation of the SubStep.
    pub fn set_latex(&mut self, latex: String) {
        self.latex = Some(latex);
    }

    /// Set the path to the diagram image representing the SubStep visually.
    pub fn set_diagram_path(&mut self, diagram_path: String) {
        self.diagram_path = Some(diagram_path);
    }

    /// Get the description of the SubStep.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the LaTeX representation of the SubStep (if available).
    pub const fn latex(&self) -> &Option<String> {
        &self.latex
    }

    /// Get the path to the diagram image (if available).
    pub const fn diagram_path(&self) -> &Option<String> {
        &self.diagram_path
    }
}