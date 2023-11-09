# Intereger Quadratics 

discriminant-integerquadratic
    .haupttiel = The discriminant (D) is calculated as D = b^2 - 4ac
    .subsitiute = Now substituting the values: \textbf{"{"} a = {$a}, b = { $b} ,c = { $c} {"}"} 
        we get \textbf{"{"} D = {$b}^2 - 4 * {$a} * {$b} {"}"} , which results in \textbf{"{"} D = {$result} {"}"}.

roots-integerquadratic
    .zero = Since  the discriminant (D) is less than 0 , the quadratic equation has no real roots
    .one = Since the discriminant (D) is equal to 0, the quadratic equation has one repeating real root; {$root }
    .two = Since the discriminant (D) is greater than 0, the quadratic equation has two distinct real roots; { $r1 } and { $r2 }

product-integerquadratic = To find the product of the roots (α and β) of a quadratic equation,
    use Vieta's Formula: \textbf {"{"} (α * β) = c/a {"}"} where \textbf {"{"} c = { $c }{"}"} and \textbf {"{"} a = { $a } {"}"}.

sum-integerquadratic = To find the sum of the roots (α and β) of a quadratic equation,
    use Vieta's Formula: \textbf {"{"} (α + β) = -b/a {"}"} where \textbf {"{"} b = { $b }{"}"} and \textbf {"{"} a = { $a } {"}"}.

axis-symmetry-integerquadratic = To find the axis of symmetry of a quadratic equation, 
    use the formula \textbf {"{"} x = -b / (2a) {"}"} where \textbf {"{"} b = { $b }{"}"} and \textbf {"{"} a = { $a } {"}"}.

concavity-integerquadratic
    .undefined = Since \textbf {"{"} a = 0 {"}"} where \textbf {"{"} a = { $a }{"}"}, the concave is undefined
    .upwards = Since \textbf {"{"} a > 0 {"}"} where \textbf {"{"} a = { $a }{"}"}, the concave is upwards
    .downwards = Since \textbf {"{"} a < 0 {"}"} where \textbf {"{"} a = { $a }{"}"}, the concave is downwards

# Terms 

algebric-term
    .add_impossible = \textbf {"{"} { $term1 } + { $term2 } {"}"}
        Since terms are not combinable as both do not contain the same variables and same variable exponents. The result is same as above.
    .add_possible =  \textbf {"{"} { $term3 } {"}"} Since terms are combinable. Add the coefficients into the above.

    .sub_impossible = \textbf {"{"} { $term1 } - { $term2 } {"}"}
        Since terms are not combinable as both do not contain the same variables and same variable exponents. The result is same as above.
    .sub_possible =  \textbf {"{"} { $term3 } {"}"} Since terms are combinable. Subtract the coefficients into the above.

    .mul = \textbf {"{"} { $term1 } * { $term2 } = { $term3 } {"}"} 
        Multiply the coefficients and powers of variables (Including lone variables, e.g., x * yx, where it's the only term with that variable.)

    .div_no_op = \textbf {"{"} { $term } / 1 = { $term } {"}"} 
        Since it's divided by 1 , we can just ignore the divide by 1

    .div_no_vars = \textbf {"{"} { $term1 } / { $term2 } = { $term3 } {"}"} 
        Divide the two numbers 

    .div_cancel_common_variables = Now cancel common variables \textit {"{"} that appear in both the top and bottom {"}"}.
        Here is the list of common variables : { $common }.
        This results in \textbf {"{"} { $term1 } / { $term2} {"}"}  
    
    .div_coefficient = Now we divide the coefficients and this results in \textbf {"{"} { $term1 } / { $term2} {"}"}  

    .single_variable_replace = We replace \textbf {"{"} { $variable } {"}"} with its value 
        and so we do \textbf {"{"} { $coefficient }({ $value }) ^ { $exponent } = { $result } {"}"} , 
        which leads to \textbf {"{"} { $term } }{"}"
    
    .multi_variable_replace = We replace \textbf {"{"} { $variables } {"}"} with their values
        and so we do \textbf {"{"} { $coefficient }{ $values } = { $result } {"}"} , 
        which leads to \textbf {"{"} { $term } }{"}"

# Expressions

algebric-expression
    .evalute = Do \textbf {"{"} { $lexpr } { $op } { $lexpr } {"}"} which is \textbf {"{"} $ans {"}"}

# Equations

algebric-equation
    .start_with_left = We start with the left side of the equation.
    .start_with_right = We start with the right side of the equation.
