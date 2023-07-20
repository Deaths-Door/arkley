mod numeric_describe;

pub use self::numeric_describe::*;

/// Represents different levels of filtering for numeric descriptions.
/// Can be used to control the level of details in the description.
#[derive(PartialEq, PartialOrd)]
pub enum FilterLevel {
    /// Basic level of filtering suitable for beginners.
    Beginner,
    /// Intermediate level of filtering for users with some experience.
    Intermediate,
    /// Advanced level of filtering for experienced users.
    Advanced,
}


/// Represents a substep in the description of numeric operations.
pub struct SubStep {
    /// Informational text for the substep.
    pub info: String,
    /// LaTeX representation of the substep (for mathematical notation).
    pub latex: String,
}

impl SubStep {
    const fn new(info: String,latex: String) -> Self {
        Self { info , latex }
    }
}

/// Represents a step in the description operations.
/// A step can consist of multiple substeps to provide detailed explanations.
pub struct Step(Vec<SubStep>);

impl Default for Step {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Step {
    fn add_substep(&mut self,substep : SubStep) {
        self.0.push(substep);
    }

    fn add_substeps(&mut self,substeps :Vec<SubStep>) {
        self.0.extend(substeps)
    }
}

/// Represents a method or operation.
/// It can be a series of steps with substeps to describe the operation in detail.
pub struct Method(Vec<Step>);