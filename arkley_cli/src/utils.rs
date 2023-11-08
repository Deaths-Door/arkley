use arkley_algebra::{
    Term, Context, 
    manipulation::{EvaluteWithValues, Find}, Equation, Expression
};

use arkley_describe::fluent_templates::{self, LanguageIdentifier};

use crate::{commands::Command, pretty_errors::try_from_with_message};

fluent_templates::static_loader! {
    static LOCALES = {
        // For now given this full path
        locales: r"C:\Users\Aarav Aditya Shah\Documents\GitHub\project-codebases\rust\arkley\arkley_algebra\translations",
        fallback_language: "en-US",
    };
}

pub fn rearrange_equation(equation: Equation,target : Term) {
    match equation.try_make_subject(target) {
        Ok(equation) => println!("Result: {equation}"),
        Err(err) => eprintln!("Error : {err}"),
    }
}

impl Command {
    pub fn command_evaluate(locale : Option<LanguageIdentifier>,input : &str,context : Context<'_>) {
        match ['=','<','>'].into_iter().any(|c| input.contains(c)) {
            true => {
                let e : Equation = try_from_with_message((input,&context));
                match locale {
                    Some(locale) => todo!("Describe for it is still penting"),
                    None => println!("Result : {}",e.evaluate_with_multiple_values(context.values()).find()),
                }        
            },
            false => {
                let e : Expression = try_from_with_message((input,&context));
                match locale {
                    Some(locale) => todo!("Describe for it is still penting"),
                    None => println!("Result : {}",e.evaluate_with_multiple_values(context.values()).find()),
                }   
            },
        }
    }

    pub fn command_rearrange(locale : Option<LanguageIdentifier>,equation : &str,context : Context<'_>,target : &str) {
        let eq : Equation = try_from_with_message((equation,&context));
        let target : Term = try_from_with_message(target);
        match locale {
            Some(locale) => todo!("Describe for it is still penting"),
            None => match eq.try_make_subject(target) {
                Ok(ok) => println!("Result : {ok}"),
                Err(err) => eprintln!("Error : {err}"),
            },
        }
    }
}
