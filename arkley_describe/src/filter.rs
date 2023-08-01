/// Represents different levels of filtering for numeric descriptions.
/// Can be used to control the level of details in the description.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FilterLevel {
    /// Basic level of filtering suitable for beginners.
    Beginner = 1,
    /// Intermediate level of filtering for users with some experience.
    Intermediate = 2,
    /// Advanced level of filtering for experienced users.
    Advanced = 3,
}

impl std::fmt::Display for FilterLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _str =  match self {
            FilterLevel::Beginner => "Basic level for beginners",
            FilterLevel::Intermediate => "Intermediate level for users with some experience",
            FilterLevel::Advanced => "Advanced level for experienced users",
        };

        write!(f,"{_str}")
    }
}

impl FilterLevel {
    /// Get the next higher filter level, or return `None` if it's already at the maximum.
    pub fn next(&self) -> Option<FilterLevel> {
        match self {
            FilterLevel::Beginner => Some(FilterLevel::Intermediate),
            FilterLevel::Intermediate => Some(FilterLevel::Advanced),
            FilterLevel::Advanced => None,
        }
    }

    /// Get the next lower filter level, or return `None` if it's already at the minimum.
    pub fn previous(&self) -> Option<FilterLevel> {
        match self {
            FilterLevel::Beginner => None,
            FilterLevel::Intermediate => Some(FilterLevel::Beginner),
            FilterLevel::Advanced => Some(FilterLevel::Intermediate),
        }
    }
}