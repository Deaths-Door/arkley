mod numeric_describe;

pub use self::numeric_describe::*;

pub enum FilterLevel {
    Beginner,
    Intermediate,
    Advanced,
}

pub struct SubStep {
    info: String,
    latex: String,
}
pub struct Step(Vec<SubStep>);

pub struct Method(Vec<Step>);