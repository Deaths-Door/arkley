- Implement reverse_tree fn for rearranger to 'work' ; give output in the expected order
- **#[deprecated(note = "Maybe this is redundant")]** for Expression::Nested
For cases like f(x) * f(x) maybe output (f(x))^2

- add equations better
- derivaes intergations , sequences
- units 
- ui / web , mobile , cli
# Done

## Discriminant:

- Calculate the discriminant (D) of a quadratic equation: D = b^2 - 4ac. 
- Use the discriminant to determine the nature of the roots:


# RUN CLIPPY
# ADD STRINGS TO THE LOCAL GENERATOR FROM dir/TODO.md 

# OPTIMZE DESCRIPTIONS FOR CASES LIKE 0 or 1 

## Quadratics 

**Note** : Requires `Describe` on equation and me knowing the maths
Solve using Vieta's Formulas  -> sum of roots && product of roots

Factor the quadratic equation to solve for the roots.
Complete the square to solve for the roots.
Use the square root property for simple quadratic equations.

Find Specific Values:

Determine the vertex (h, k) of a quadratic equation in vertex form (y = a(x - h)^2 + k).
Find the axis of symmetry, which is x = h.
Calculate the maximum or minimum value of the quadratic function.
 
Quadratic Inequalities:

Solve quadratic inequalities, which involve expressions like ax^2 + bx + c > 0 or ax^2 + bx + c < 0.

Completing the Square:

Convert a quadratic equation to vertex form by completing the square.

Finding "k":

Given a quadratic equation in the form y = ax^2 + bx + c, find the value of "k" for a specific x or y value.h.

The `Quadratic` trait is a good starting point for working with quadratic equations. Here are some additional methods and functionalities you can consider adding to this trait:

1. **Solving Quadratic Equation:** Implement a method to directly solve the quadratic equation based on the coefficients `a`, `b`, and `c`. This method can return the solutions as real numbers or complex numbers.

2. **Vertex Form Conversion:** Implement methods to convert the quadratic equation from the standard form (`ax^2 + bx + c`) to the vertex form (`a(x-h)^2 + k`) and vice versa.

3. **Completing the Square:** Add a method to complete the square for a given quadratic equation, providing the expression in the form `(x-h)^2 = k`.

4. **Finding Vertex:** Implement a method to calculate the vertex (h, k) of the parabola represented by the quadratic equation.

5. **Axis of Symmetry:** Add a method to find the axis of symmetry for the parabola.

6. **Derivative:** If your quadratic equation represents a function, consider adding a method to calculate the derivative of the quadratic function.

7. **Minimum or Maximum Value:** Include a method to determine whether the quadratic function has a minimum or maximum value and calculate its magnitude.

8. **Factorization:** Implement a method to factor the quadratic equation into its linear factors.

9. **Quadratic Formula Derivation:** Add a method to derive the quadratic formula `x = (-b ± √(b^2 - 4ac)) / (2a)`.

10. **Root Intervals:** Calculate and return the intervals where the roots of the quadratic equation exist.

11. **Concavity:** Determine the concavity of the parabola (whether it opens upward or downward) based on the coefficient `a`.

12. **Complex Roots Handling:** If the quadratic equation may have complex roots, provide methods to work with complex numbers when calculating roots.

13. **Equation Validation:** Check whether the given equation is a valid quadratic equation (i.e., a should not be zero).

The specific methods you choose to implement will depend on your use case and the level of functionality you want to provide for working with quadratic equations. Consider which methods would be most useful to your application or library users.