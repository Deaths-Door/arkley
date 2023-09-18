use fluent_templates::{LanguageIdentifier,lazy_static::lazy_static};

use std::collections::HashMap;
/// Enum representing supported languages.
#[derive(Debug, PartialEq, Eq,Hash)]
pub enum SupportedLanguages {
    /// English language.
    English,
    /// German language.
    Deustch,
    /// French language.
    Francais,
    /// Hindi language.
    Hindi,
}

lazy_static! {
    pub static ref LANGUAGE_MAP: std::collections::HashMap<SupportedLanguages, LanguageIdentifier> = {
        /*let mut map = std::collections::HashMap::from();
        map.insert(SupportedLanguages::English, "en-US".parse().expect("ERROR PARSING LanguageIdentifier"));
        map.insert(SupportedLanguages::Deustch, "de-DE".parse().expect("ERROR PARSING LanguageIdentifier"));
        map.insert(SupportedLanguages::Francais, "fr-FR".parse().expect("ERROR PARSING LanguageIdentifier"));
        map.insert(SupportedLanguages::Hindi, "hi-IN".parse().expect("ERROR PARSING LanguageIdentifier"));
        map*/
    };
}