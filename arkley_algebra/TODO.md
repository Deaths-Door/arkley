create solve for equations
describe ops with expressions
manipulation describe
add in reamde - this crate is extendable with custom types as well
derive - Minimum or Maximum Value / intergate
add to main readme + each crate readme that used describe feature that each dir contains the translations dir with translations  + create buildscript for 'copying' this on to new projcet
add algebric quadractics / + from expression form equation for both alg + inter quad
Calculate the maximum or minimum value of the quadratic function.

describe sum_of_roots and product of roots methods from quadratic trait
- **#[deprecated(note = "Maybe this is redundant")]** for Expression::Nested
For cases like f(x) * f(x) maybe output (f(x))^2

- add equations better
- derivaes intergations , sequences
- units 
- ui / web , mobile , cli

todo so convaticy descrbie
add tryfrom for interger quads 
impl<T> TryFrom<Expression> for IntegerQuadratic<T> where T: Num + Clone + From<u8> + From<Number> {
    type Error = QuadraticError;
    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        
    }   
}



#[cfg(feature="equation")]
impl<T> TryFrom<Equation> for IntegerQuadratic<T> where T: Num + Clone + From<u8> + From<Number> {
    type Error = QuadraticError;
    fn try_from(value: Equation) -> Result<Self, Self::Error> {
        let equation = value.try_make_subject(0.into()).unwrap(); // cuz it will always succed in rearanaing into 0 


        equation.
    }   
}
## Quadratics 
Factor the quadratic equation to solve for the roots.
Complete the square to solve for the roots.
Find Specific Values:
 
Completing the Square:

Convert a quadratic equation to vertex form by completing the square.

Finding "k":

Given a quadratic equation in the form y = ax^2 + bx + c, find the value of "k" for a specific x or y value.

2. **Vertex Form Conversion:** Implement methods to convert the quadratic equation from the standard form (`ax^2 + bx + c`) to the vertex form (`a(x-h)^2 + k`) and vice versa.

13. **Equation Validation:** Check whether the given equation is a valid quadratic equation (i.e., a should not be zero).