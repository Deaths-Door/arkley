# arkley-alebgra

This crate is designed to empower users with algebraic tools tailored for high school algebra problems. Geared towards simplicity and functionality, this crate provides a collection of structs and functions that facilitate the manipulation and resolution of algebraic expressions, equations, and operations commonly encountered in basic algebra.

## General Features
- **Expression Evaluator**: Evaluate algebraic expressions effortlessly, providing a solid foundation for various algebraic tasks.
- **Variable Support**: Utilize variables, assignments, and statement chaining within expressions. Assignments to variables are stored in a context, preserving their values for future evaluations.
- **User-Defined Functions**: Define and incorporate custom functions within expressions, extending the versatility of algebraic manipulations.
- **User-Defined Tags** : Define things like `speed_of_light` and give it an 'value' of **3*10^8**
- **Parameterizable Formulas**: Compile formulas with parameters using variables. 
- **Function Definitions**: Define arbitrary functions using the Function instance, allowing for more complex and tailored algebraic operations.
- **Context Flexibility:** The context stores mappings for values, tags, and, if the "function" feature is enabled, functions. Value and function mappings are stored independently.

## Note

While we strive to cover as many scenarios as possible, the complexity of mathematics means that achieving 100% accuracy in all cases and operations can be challenging. If you come across any issues or inaccuracies in the library, please don't hesitate to report them so that we can work on improving it. Your feedback is invaluable in helping us make the necessary enhancements.

### Mathematical Features

### Context
The `Context` struct provides a versatile environment for storing mappings in hash maps. This context plays a crucial role in managing variables, user-defined functions, and tags within algebraic expressions during the parsing phase.

- **Value and Function Mappings**: Mappings are stored independently, allowing for the existence of a function and a value with the same identifier.
- **Variable Assignments**: Assignments to variables are stored in the context during the parsing phase.
- **Variable Retrieval**: When variables are read later in the expression, their values are retrieved from the context, ensuring consistency in subsequent evaluations.
- **Variable Subsitution**

### Equations and Inequalities

The `Equation` struct is a versatile tool designed to handle equations and inequalities. This powerful component provides functionalities for evaluation, rearrangement, solving, and more.

- **Evalulation** : Evaluate equations with specific variable assignments:
- **Rearrangement** : Rearrange equations by isolating variables:
- **Solving** : Solve equations for specific variables:
- **Inequalities** : Handle inequalities and evaluate them:

### Quadratics

This crate introduces the `Quadratic` trait, providing a powerful set of methods for working with quadratic equations and exploring their properties , extendable with custom types
- **Discriminant Calculation** : Calculate the discriminant of a quadratic equation
- **Roots Calculation** : Determine the roots of a quadratic equation using the discriminant
- **Sum and Product of Roots** : Calculate the sum and product of the roots using Vieta's Formulas
- **Axis of Symmetry** : Determine the axis of symmetry for a quadratic equation
- **Concavity Determination** :Understand the concavity of the parabola based on the coefficient a

### Describe Operations
The `describe` feature empowers you to obtain detailed descriptions of various algebraic elements, enhancing your understanding of equations and expressions. By using the describe feature, you can explore and visualize the properties of quadratic equations, expressions, and more.

**To setup describe feature** : Enable the `describe` feature and then refer to [this link](https://github.com/Deaths-Door/arkley/tree/main/arkley_describe/README.md)


## Documentation

For detailed documentation, please visit the [crate's documentation](https://docs.rs/arkley_algebra)
