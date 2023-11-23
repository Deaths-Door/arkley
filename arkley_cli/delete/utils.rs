use arkley_algebra::{
    Term, Context, 
    manipulation::{EvaluteWithValues, Find}, Equation, Expression
};

use arkley_describe::fluent_templates::{self, LanguageIdentifier};

use crate::{commands::{Command, QuadraticsCommands}, pretty_errors::try_from_with_message};

pub enum ExpressionOrEquation {
    Eq(Equation),
    Expr(Expression)
}

impl From<(&str,&Context<'_>)> for ExpressionOrEquation {
    fn from((input,context): (&str,&Context<'_>)) -> Self {
        match ['=','<','>'].into_iter().any(|c| input.contains(c)) {
            true => ExpressionOrEquation::Eq(try_from_with_message((input,context))),
            false =>ExpressionOrEquation::Expr(try_from_with_message((input,context))),
        }
    }
}

fluent_templates::static_loader! {
    static LOCALES = {
        // For now given this full path
        locales: r"C:\Users\Aarav Aditya Shah\Documents\GitHub\project-codebases\rust\arkley\arkley_algebra\translations",
        fallback_language: "en-US",
    };
}

#[deprecated]
pub fn rearrange_equation(equation: Equation,target : Term) {
    match equation.try_make_subject(target) {
        Ok(equation) => println!("Result: {equation}"),
        Err(err) => eprintln!("Error : {err}"),
    }
}

#[deprecated]
fn parse_expression_or_equation(input : &str,context : &Context<'_>,eq : impl FnOnce(Equation), expr : impl FnOnce(Expression)) {
    match ['=','<','>'].into_iter().any(|c| input.contains(c)) {
        true => {
            let e : Equation = try_from_with_message((input,context));
            (eq)(e)
        },
        false =>  {
            let e : Expression = try_from_with_message((input,context));
            (expr)(e)
        }
    }
}

#[deprecated]
impl Command {
    pub fn command_evaluate(locale : &Option<LanguageIdentifier>,input : &str,context : &Context<'_>) {
        parse_expression_or_equation(input, context, 
        |e| match locale {
            Some(locale) => todo!("Describe for it is still penting"),
            None => println!("Result is {}",e.evaluate_with_multiple_values(context.values()).find()),
        },
         |e| match locale {
            Some(locale) => todo!("Describe for it is still penting"),
            None => println!("Result is {}",e.evaluate_with_multiple_values(context.values()).find()),
        } )
    }

    pub fn command_rearrange(locale : Option<LanguageIdentifier>,equation : &str,context : &Context<'_>,target : &str) {
        let eq : Equation = try_from_with_message((equation,context));
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

#[deprecated]
impl QuadraticsCommands {
    pub fn handle(self,context : &Context<'_>,locale : &Option<LanguageIdentifier>) {
        macro_rules! logic {
            (@inner => $find : ident, $input : expr , $context : expr , $locale : expr) => {
                // use $find .find()
                parse_expression_or_equation($input, context, 
                    |e| match locale {
                    Some(locale) => todo!("Describe for it is still penting"),
                    None => println!("Result is {}",todo!()),
                },
                |e| match locale {
                    Some(locale) => todo!("Describe for it is still penting"),
                    None => println!("Result is {}",todo!()),
                })
            };


            (enter => $find : ident, $input : expr) => {
                logic!(@inner => $find,& $input,context,locale)
            }
        }
        
        match self {
            QuadraticsCommands::Discriminant { expression_or_equation } => logic!(enter => discriminant,expression_or_equation),
            QuadraticsCommands::Roots { expression_or_equation } => logic!(enter => roots,expression_or_equation),
            QuadraticsCommands::SumOfRoots { expression_or_equation } => logic!(enter => sum_of_roots,expression_or_equation),
            QuadraticsCommands::ProductOfRoots { expression_or_equation } => logic!(enter => product_of_roots,expression_or_equation),
            QuadraticsCommands::AxisOfSymmetry { expression_or_equation } => logic!(enter => axis_of_symmetry,expression_or_equation),
            QuadraticsCommands::Concavity { expression_or_equation } => logic!(enter => concavity,expression_or_equation),
        }
    }
}