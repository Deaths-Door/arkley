use std::{
    fs::{
    create_dir_all,
    create_dir, File
    }, 
    collections::HashMap,
    io::Write
};

/// Generates resource files for different languages based on a given directory and translations.
///
/// This function creates resource files for different languages based on the provided
/// `directory` path and a hashmap of `translations`. Each language's translations are
/// stored in a separate directory under `directory`, and the values are written to a
/// `main.ftl` file in each language directory.
///
/// # Arguments
///
/// * `directory` - A path to the directory where the resource files will be generated.
/// * `translations` - A hashmap containing language identifiers as keys and vectors of
///   translations as values. Each vector should contain translations for a specific language.
///
/// # Errors
///
/// This function returns an error if the number of translations for a language does not
/// match the number of language identifiers.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// let directory = "locales";
/// let mut translations = HashMap::new();
/// translations.insert("en-US", vec!["English content 1", "English content 2"]);
/// translations.insert("fr"   , vec!["French content 1",  "French content 2"]  );
///
/// match generate_resources(directory, translations) {
///     Ok(_) => println!("Resource files generated successfully."),
///     Err(err) => eprintln!("Error: {}", err),
/// }
/// ```
pub fn generate_resources<P : AsRef<std::path::Path>>(directory : P, translations: HashMap<&str, Vec<&str>>) -> Result<(), std::io::Error> {
    create_dir_all(directory)?;

    for (lang_identifier, values) in translations.iter() {
        let lang_dir = directory.as_ref().join(lang_identifier);

        create_dir(&lang_dir)?;
        let file_path = lang_dir.join("main.ftl");

        if values.len() != translations.keys().len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Number of values must match the number of language identifiers.",
            ));
        }

        // Write the values to the file
        let mut file = File::create(file_path)?;
        for value in values {
            file.write_all(value.as_bytes())?;
        }
    }

    Ok(())
}