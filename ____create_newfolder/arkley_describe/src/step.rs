use crate::SubStep;

/// Represents a step in the description of an operation.
#[derive(Debug)]
pub struct Step {
    /// Description of the step.
    description: String,

    /// List of sub-steps for the step.
    sub_steps: Vec<SubStep>,
}

impl Step {
    /// Creates a new `Step` instance with the provided title and description.
    ///
    /// # Arguments
    ///
    /// * `title`: The title of the step as a `String`.
    /// * `description`: The description of the step as a `String`.
    ///
    /// # Returns
    ///
    /// A new `Step` object with the given `title` and `description`. The `sub_steps` field
    /// is initialized as an empty vector.
    pub const fn new(description: String) -> Step {
        Self { description , sub_steps : Vec::new() }
    }

    /// Inserts the provided string slice into the existing description of the `Step`.
    ///
    /// # Arguments
    ///
    /// * `ds`: A string slice (`&str`) to be appended to the existing description.
    pub fn insert_to_description(&mut self,ds : &str) {
        self.description += ds;
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