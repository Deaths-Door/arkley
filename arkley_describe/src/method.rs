use crate::Step;

/// Represents a method or operation.
/// It can be a series of steps with substeps to describe the operation in detail.
#[derive(Debug)]
pub struct Method {
    /// Name of the method or operation.
    name: String,
    /// Steps of the method.
    steps: Vec<Step>,
}

impl Method {
    /// Creates a new instance of `Method` with the given name and an empty vector for steps.
    pub const fn new(name: String) -> Self {
        Self {
            name,
            steps: Vec::new(),
        }
    }

    /// Adds a step to the method.
    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }
}