use std::collections::HashMap;

use arkley_describe::{
    Describe,
    Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}
};

use num_notation::Number;


use crate::{ 
    Term,
    manipulation::{
        SingleVariableReplacements,
        MultipleVariableReplacements
    }
};

macro_rules! impl_trait {
    (term single => $($value : ty),*) => {
        $(
            impl Describe for SingleVariableReplacements<Term,$value> {
                fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    let (variable,value) = (&self.variable, self.value);

                    let mut steps = Vec::new();
                    let mut term = self.source;

                    if let Some(exponent) = term.variables.remove(variable) {
                        let mut args = HashMap::from([
                            ("variable",variable.to_string().into()),
                            ("coefficient",term.coefficient.to_string().into()),
                            ("value",value.to_string().into()),
                            ("exponent",exponent.to_string().into())
                        ]);

                        term.coefficient *= (value as f64).powf(f64::from(exponent));
                        
                        args.insert("result",term.coefficient.to_string().into());
                        args.insert("term",term.to_string().into());

                        let s = resources.lookup_single_language(lang,"algebric-term.single_variable_replace",Some(&args))?;
                        steps.push(s);
                    }

                    Some(steps)
                }
            }
        )*
    };

    (term number single) => {
            impl Describe for SingleVariableReplacements<Term,Number> {
                fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    let variable = &self.variable;
                    let value = self.value;

                    let mut steps = Vec::new();
                    let mut term = self.source;

                    if let Some(exponent) = term.variables.remove(variable) {
                        let mut args = HashMap::from([
                            ("variable",variable.to_string().into()),
                            ("coefficient",term.coefficient.to_string().into()),
                            ("value",value.to_string().into()),
                            ("exponent",exponent.to_string().into())
                        ]);

                        term.coefficient *= f64::from(value).powf(f64::from(exponent));
                        
                        args.insert("result",term.coefficient.to_string().into());
                        args.insert("term",term.to_string().into());

                        let s = resources.lookup_single_language(lang,"algebric-term.single_variable_replace",Some(&args))?;
                        steps.push(s);
                    }

                    Some(steps)
                }
            }  
    };

    (term multi => $($value : ty),*) => {
        $(
            impl Describe for MultipleVariableReplacements<'_,Term,$value> {
                fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    let mut steps = Vec::new();

                    let (mut term,values) = (self.source,self.values);

                    let mut args = HashMap::from([ ("coefficient",term.coefficient.to_string().into()) ]);

                    let (mut variables_vec,mut values_vec) = (Vec::new(),Vec::new());

                    for (k,v) in values.iter() {
                        if let Some(exponent) = term.variables.remove(k) {
                            variables_vec.push(k);
                            values_vec.push(format!("({v})^({exponent})"));
                            term.coefficient *= (*v as f64).powf(f64::from(exponent));
                        }
                    }

                    args.insert("variables",variables_vec.into_iter().fold(String::new(),|mut curr,nxt| {
                        curr.push(*nxt);
                        curr.push(',');
                        curr
                    }).into());
                    args.insert("values",values_vec.join(""));

                    args.insert("result",term.coefficient.to_string().into());
                    args.insert("term",term.to_string().into());

                    let s = resources.lookup_single_language(lang,"algebric-term.single_variable_replace",Some(&args))?;
                    steps.push(s);

                    Some(steps)   
                }
            }
        )*
    };

    
}

impl_trait!(term single => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
impl_trait!(term number single);
impl_trait!(term multi => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
