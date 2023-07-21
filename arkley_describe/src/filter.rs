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