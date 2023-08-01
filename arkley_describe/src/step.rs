use crate::SubStep;

/// Represents a step in the description of an operation.
#[derive(Debug)]
pub struct Step {
    /// Title of the step.
    title: String,

    /// Description of the step.
    description: String,

    /// List of sub-steps for the step.
    sub_steps: Vec<SubStep>,
}

impl Step {
    pub const fn new(title: String,description: String) -> Step {
        Self { title,description , sub_steps : Vec::new() }
    }

    pub fn insert_to_description(&mut self,ds : &str) {
        self.description += ds;
    }

    /// Get the title of the Step.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the description of the Step.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the list of sub-steps for the Step.
    pub fn substeps(&self) -> &[SubStep] {
        &self.sub_steps
    }

    /// Adds a single substep to the step.
    pub fn add_substep(&mut self, substep: SubStep) {
        self.sub_steps.push(substep);
    }
}