use std::{collections::HashMap, hash::Hash, vec};

use arkley_describe::{
    DescribeAdd, Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}, DescribeSub, DescribeMul, DescribeDiv
};

use crate::Term;

impl DescribeAdd for Term {
    fn describe_add(self,other:Self,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let mut args = HashMap::new();

        let string = match self.is_combinable_with(&other) {
            true => {
                args.insert("term1", self.to_string().into());
                args.insert("term2", other.to_string().into());
                resources.lookup_single_language(lang,"algebric-term.add_possible",Some(&args))
            },
            false => {
                args.insert("term3", self.force_add_terms(other).to_string().into());
                resources.lookup_single_language(lang,"algebric-term.add_impossible",Some(&args))
            }
        }?;

        vec![string].into()
    }
}

impl DescribeSub for Term {
    fn describe_sub(self,other:Self,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let mut args = HashMap::new();

        let string = match self.is_combinable_with(&other) {
            true => {
                args.insert("term1", self.to_string().into());
                args.insert("term2", other.to_string().into());
                resources.lookup_single_language(lang,"algebric-term.sub_possible",Some(&args))
            },
            false => {
                args.insert("term3", self.force_add_terms(other).to_string().into());
                resources.lookup_single_language(lang,"algebric-term.sub_impossible",Some(&args))
            }
        }?;

        vec![string].into()
    }
}

impl DescribeMul for Term {
    fn describe_mul(self,other:Self,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let args = HashMap::from([
            ("term1", self.to_string().into()),
            ("term2", other.to_string().into()),
            ("term3", (self * other).to_string().into())
        ]);

        let string = resources.lookup_single_language::<&str>(lang,"algebric-term.mul",Some(&args))?;
        vec![string].into()
    }
}

impl DescribeDiv for Term {
    fn describe_div(self,other:Self,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        if other.coefficient == 1 && other.variables.is_empty() {
            let args = HashMap::from([
                ("term",self.to_string().into())
            ]);
            return vec![
                resources.lookup_single_language(lang, "algebric-term.div_no_op", Some(&args))?
            ].into();
        };


        if self.variables.is_empty() && other.variables.is_empty() {
            let args = HashMap::from([
                ("term1",self.coefficient.to_string().into()),
                ("term2",other.coefficient.to_string().into()),
                ("term3",(self.coefficient / other.coefficient).to_string().into())
            ]);

            return vec![
                resources.lookup_single_language(lang, "algebric-term.div_no_vars", Some(&args))?
            ].into()
        }


        vec![r"\\textbf{TODO THIS}".to_owned()].into()
    }
}