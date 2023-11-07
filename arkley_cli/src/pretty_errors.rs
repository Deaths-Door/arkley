use std::{process::exit, fmt::Display};

use arkley_algebra::{Context, Equation, Term};
use rustyline::DefaultEditor;

pub fn new_default_editor() -> DefaultEditor {
    match DefaultEditor::new() {
        Ok(e) => e,
        Err(error) => {
            eprintln!("Error creating playground {error}");
            exit(1)
        },
    }
}

pub fn try_from_with_message<T : TryFrom<I>,I>(input : I) -> T {
    match T::try_from(input) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("Sadly given input is invalid , consider inputing a valid input");
            exit(1)
        },
    }
}