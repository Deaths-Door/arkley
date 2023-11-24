use std::{fmt::Display, process::exit, path::PathBuf, sync::OnceLock};

use arkley_algebra::{Equation, Expression, Context, quadratics::{IntegerQuadratic,Quadratic, Nature}, manipulation::Find, Term};
use arkley_describe::{*, fluent_templates::{LanguageIdentifier,static_loader}};
use crate::command::{QuadraticsCommands, QuadraticArguments};

static_loader! {
    static LOCALES = {
        // TODO : Change this later
        locales: r"C:\Users\Aarav Aditya Shah\Documents\GitHub\project-codebases\rust\arkley\arkley_algebra\translations",
        fallback_language: "en-US",
    };
}

pub static CURRENT_EXE_DIR : OnceLock<PathBuf> = OnceLock::new();


pub enum ExpressionOrEquation {
    Eq(Equation),
    Expr(Expression)
}

impl From<(&str,&Context<'_>)> for ExpressionOrEquation {
    fn from((input,context): (&str,&Context<'_>)) -> Self {
        match ['=','<','>'].into_iter().any(|c| input.contains(c)) {
            true => ExpressionOrEquation::Eq(try_from_with_message((input,context))),
            false => ExpressionOrEquation::Expr(try_from_with_message((input,context))),
        }
    }
}

fn try_from_with_message<T : TryFrom<I>,I>(input : I) -> T {
    match T::try_from(input) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("Sadly given input is invalid , consider inputing a valid input");
            exit(1)
        },
    }
}

fn find_or_describe<T,O,D>(
    locale : &Option<LanguageIdentifier>,
    item : T,
    print : impl FnOnce(O) -> D 
) where 
    T : Describe + Find<Output = O> , 
    D : Display {
    match locale {
        Some(locale) => {
            let uuid = uuid::Uuid::new_v4().to_string();
            let mut path = CURRENT_EXE_DIR.get().unwrap().clone();
            path.push(uuid);

            write_description_to_file(
                &LOCALES,
                &locale, 
                item,
                path,
                || eprintln!("Check whether required resources are availiable"), 
            ).expect("Error in proccessing file");

            todo!("Convert this to a pdf or html and show it to user")
        },
        None => println!("{}",print(item.find())),
    }
}

pub fn evaluate_handler(expr_eq : String,locale : &Option<LanguageIdentifier>,context : &Context<'_>) {
    todo!("Describe for it not done yet")
   /*  
   
impl ExpressionOrEquation {
   fn handle(self,expr : impl FnOnce(Expression),eq : impl FnOnce(Equation)) {
        match self {
            ExpressionOrEquation::Eq(v) => eq(v),
            ExpressionOrEquation::Expr(v) => expr(v),
        }
   }
}
   ExpressionOrEquation::from((expr_eq.as_str(),context))
        .handle(
            |v| 
            /*|v| find_or_describe(
                locale, 
                |v : Expression| v.evaluate_with_multiple_values(context.values()),
                |v| format!("{v}")),
            |v| find_or_describe(
                locale, 
                v,
                |v| format!("{v}")),*/
        )*/
}

pub fn command_rearrange(locale : &Option<LanguageIdentifier>,equation : &str,context : &Context<'_>,target : &str) {
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

impl QuadraticsCommands {
    pub fn handle_subcommands(
        self,
        input : Option<String>,
        arguments : QuadraticArguments,
        context : &Context<'_>,
        locale : Option<LanguageIdentifier>
    ) {
        #[deprecated(note="Placeholder till quadratcs with algebric types is implemented")]
        const TEMP : IntegerQuadratic<f64> = IntegerQuadratic::new(0.0, 0.0, 0.0);

        match self {
            QuadraticsCommands::Discriminant => self.input_or_arguments(
                context, 
                input, 
                arguments,
                &locale, 
                |v| TEMP.discriminant(),
                |v| v.parse_abc().discriminant(),
                |v: f64| format!("The discriminant is {v}")
            ),
            QuadraticsCommands::Roots => self.input_or_arguments(
                context, 
                input, 
                arguments,
                &locale, 
                |v| TEMP.roots(),
                |v| v.parse_abc().roots(),
                |v| match v {
                    Nature::DistinctRealRoots(r1,r2) => format!("The 2 real roots are {r1} and {r2}"),
                    Nature::SingleRoot(r) => format!("The single repeating real root is {r}"),
                    Nature::NoRealRoots => format!("There are no real roots for input"),
                }
            ),
            QuadraticsCommands::SumOfRoots => self.input_or_arguments(
                context, 
                input, 
                arguments,
                &locale, 
                |v| TEMP.sum_of_roots(),
                |v| v.parse_ab().sum_of_roots(),
                |v: f64| format!("The sum of roots is {v}")

            ),
            QuadraticsCommands::ProductOfRoots => self.input_or_arguments(
                context, 
                input, 
                arguments,
                &locale, 
                |v| TEMP.product_of_roots(),
                |v| v.parse_ac().product_of_roots(),
                |v: f64| format!("The product of roots is {v}")

            ),
            QuadraticsCommands::AxisOfSymmetry => self.input_or_arguments(
                context, 
                input, 
                arguments,
                &locale, 
                |v| TEMP.axis_of_symmetry(),
                |v| v.parse_ab().axis_of_symmetry(),
                |v: f64| format!("The axis of symmetry is {v}")

            ),
            QuadraticsCommands::Concavity => self.input_or_arguments(
                context, 
                input, 
                arguments,
                &locale, 
                |v| TEMP.concavity(),
                |v| v.parse_a().concavity(),
                |v| format!(r"The concavity is {v}")
            ),
        }
    }

    fn input_or_arguments<T1,O,T2,F1,F2,D>(
        self,
        context : &Context<'_>, 
        input : Option<String>,
        arguments : QuadraticArguments,
        locale : &Option<LanguageIdentifier>,
        expreq : F1 ,
        quad : F2,
        print : impl FnOnce(O) -> D 
    ) where 
        T1: Describe + Find<Output = O>, 
        T2: Describe + Find<Output = O> , 
        F1 : FnOnce(ExpressionOrEquation) -> T1 , 
        F2 : FnOnce(QuadraticArguments) -> T2,
        D : Display {
        if let Some(i) = input {
            find_or_describe(locale,expreq(ExpressionOrEquation::from((i.as_str(),context))),print);
            return 
        };
        
        find_or_describe(locale, quad(arguments),print)
    }
}

static MSG : &str = "Unfornuately a required argument is not provided";
static PARSE_ERR : &str = "Error parsing into float";

impl QuadraticArguments {
    fn parse_abc(self) -> IntegerQuadratic<f64> {
        let (a,b,c) = (
            self.a.expect(MSG).parse().expect(PARSE_ERR),
            self.b.expect(MSG).parse().expect(PARSE_ERR),
            self.c.expect(MSG).parse().expect(PARSE_ERR)
        );

        IntegerQuadratic::new(a, b, c)
    }

    fn parse_ab(self) -> IntegerQuadratic<f64> {
        let (a,b) = (
            self.a.expect(MSG).parse().expect(PARSE_ERR),
            self.b.expect(MSG).parse().expect(PARSE_ERR),
        );

        IntegerQuadratic::new(a, b, 0.0)
    }


    fn parse_ac(self) -> IntegerQuadratic<f64> {
        let (a,c) = (
            self.a.expect(MSG).parse().expect(PARSE_ERR),
            self.c.expect(MSG).parse().expect(PARSE_ERR),
        );

        IntegerQuadratic::new(a, 0.0,c)
    }

    fn parse_a(self) -> IntegerQuadratic<f64> {
        let a =  self.a.expect(MSG).parse().expect(PARSE_ERR);

        IntegerQuadratic::new(a, 0.0, 0.0)
    }
}