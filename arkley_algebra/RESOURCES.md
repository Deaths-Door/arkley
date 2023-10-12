**discriminant-integerquadratic** :
```
.haupttiel = The discriminant (D) is calculated as D = b^2 - 4ac
.subsitiute = subsitiute = Now substituting the values: \textbf{"{"} a = {$a}, b = { $b} ,c = { $c} {"}"} 
    we get \textbf{"{"} D = {$b}^2 - 4 * {$a} * {$b} {"}"} , which results in \textbf{"{"} D = {$result} {"}"} .
```

**roots-integerquadratic** :
```
.zero = Since  the discriminant (D) is less than 0 , the quadratic equation has no real roots
.one = Since the discriminant (D) is equal to 0, the quadratic equation has one repeating real root; {$root }
.two = Since the discriminant (D) is greater than 0, the quadratic equation has two distinct real roots; { $r1 } and { $r2 }
```

**algebric-term**
```
.add_impossible = \textbf {"{"} { $term1 } + { $term2 } {"}"}
    Since terms are not combinable as both do not contain the same variables and same variable exponents. The result is same as above.
.add_possible =  \textbf {"{"} { $term3 } {"}"} Since terms are combinable. Add the coefficients into the above.
 
######### 

.sub_impossible = \textbf {"{"} { $term1 } - { $term2 } {"}"}
    Since terms are not combinable as both do not contain the same variables and same variable exponents. The result is same as above.
.sub_possible =  \textbf {"{"} { $term3 } {"}"} Since terms are combinable. Subtract the coefficients into the above.

#########

.mul = \textbf {"{"} { $term1 } * { $term2 } = { $term3 } {"}"} 
    Multiply the coefficients and powers of variables (Including lone variables, e.g., x * yx, where it's the only term with that variable.)

#########

.div_no_op = \textbf {"{"} { $term } / 1 = { $term } {"}"} 
    Since it's divided by 1 , we can just ignore the divide by 1

.div_no_vars = \textbf {"{"} { $term1 } / { $term2 } = { $term3 } {"}"} 
    Divide the two numbers 

```