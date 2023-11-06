use std::process::exit;

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

// TODO : Make this more generic or improve it in general

pub fn equation_try_from_with_message<'a>(input : &str,context : &Context<'a>) -> Equation {
    match Equation::try_from((input,context)) {
        Ok(eq) => eq,
        Err(_) => {
            eprintln!("Sadly given equation ({input}) is invalid , consider inputing a valid equation");
            exit(1)
        },
    }
}
pub fn term_try_from_with_message(input : &str) -> Term {
    match Term::try_from(input) {
        Ok(term) => term,
        Err(_) => {
            eprintln!("Sadly given term ({input}) is invalid , consider inputing a valid term");
            exit(1)
        },
    }
}