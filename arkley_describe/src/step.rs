use crate::SubStep;

/// Represents a step in the description operations.
/// A step can consist of multiple substeps to provide detailed explanations.
#[derive(Debug)]
pub struct Step(String,Vec<SubStep>);

impl Step {
    /// Creates a new instance of `Step` with the given header information.
    pub const fn new(header_info: String) -> Self {
        Self(header_info, Vec::new())
    }

    /// Adds a single substep to the step.
    pub fn add_substep(&mut self, substep: SubStep) {
        self.1.push(substep);
    }

    /// Adds multiple substeps to the step.
    pub fn add_substeps(&mut self, substeps: Vec<SubStep>) {
        self.1.extend(substeps);
    }
}