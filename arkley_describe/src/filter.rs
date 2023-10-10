/// Represents different levels of filtering for numeric descriptions.
/// Controls the level of detail and explanation in the description.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FilterLevel {
    /// Provides a basic, beginner-friendly description.
    /// Assumes limited prior experience.
    Beginner,
    
    /// Offers an intermediate level of detail in the description.
    /// Suitable for users with some prior experience.
    Intermediate,
    
    /// Provides an advanced and detailed description.
    /// Assumes a high level of prior experience.
    Advanced,
}
