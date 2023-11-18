use std::collections::HashMap;

use arkley_describe::{
    Describe,
    Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}
};

use crate::{Term, Expression, Equation, ArithmeticOperation};

use super::{Evaluate, EvaluateNoValues};

impl Describe for EvaluateNoValues<Term> {
    fn describe(self,_:&StaticLoader,_: &LanguageIdentifier) -> Option<Steps> {
        Some(vec![])
    }
}

impl ArithmeticOperation {
    fn operate_on_describe(&self,
        resource:&StaticLoader,
        lang: &LanguageIdentifier,
        left : Expression,
        right : Expression,
        steps : &mut Steps
    ) -> Option<Expression> {
        let mut args = HashMap::from([
            ("lexpr",left.to_string().into()),
            ("rexpr",right.to_string().into()),
            ("op",self.to_string().into()),
        ]);

        let ans = self.operate_on(left,right);

        args.insert("ans",ans.to_string().into());

        let s = resource.lookup_single_language(lang, "algebric-expression.evalute", Some(&args))?;
        steps.push(s);

        Some(ans)    
    }
}

impl Expression {
    fn show(self,resource:&StaticLoader,lang: &LanguageIdentifier,steps : &mut Steps) -> Option<Expression> {
        match self {
            Expression::Term(_) => Some(self),
            Expression::Binary { operation, left, right } => 
                operation.operate_on_describe(resource,lang,*left,*right,steps),
            Expression::Function(func) => todo!()
        }   
    }
}

impl Describe for EvaluateNoValues<Expression> {
    fn describe(self,resource:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let mut steps = Vec::new();
        self.0.show(resource, lang,&mut steps);
        Some(steps)
    }
}

impl Describe for EvaluateNoValues<Equation> {
    fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let mut steps = Vec::new();

        let left_description = self.0.left.evaluate().describe(resources, lang)?;
        let right_description = self.0.right.evaluate().describe(resources, lang)?;

        if !left_description.is_empty() {
            steps.push(resources.lookup_single_language::<&str>(lang, "algebric-equation.start_with_left",None)?);
            steps.extend(left_description.into_iter());
        }

        if !right_description.is_empty() {
            steps.push(resources.lookup_single_language::<&str>(lang, "algebric-equation.start_with_right",None)?);
            steps.extend(right_description.into_iter());
        }

        Some(steps)
    }
}