mod interger;

mod error;

mod discriminant;
mod roots;
mod sum;
mod product;
mod axis_symmetry;
mod concavity;

pub use interger::*;
pub use error::*;

pub use discriminant::*;
pub use roots::*;
pub use sum::*;
pub use product::*;
pub use axis_symmetry::*;
pub use concavity::*;


/// A trait for working with quadratic equations and their properties.
///
/// The `Quadratic` trait provides a set of methods for working with quadratic
/// equations, including calculating the discriminant and determining the nature
/// of roots. It allows for easy extension with custom types.
pub trait Quadratic<T> : Sized {
    /// Calculates the discriminant of the quadratic equation.
    ///
    /// The discriminant of a quadratic equation `ax^2 + bx + c` is given by `D = b^2 - 4ac`
    ///
    /// # Returns
    ///
    /// The discriminant value of the quadratic equation.
    fn discriminant(self) -> Discriminant<T>;

    /// Calculates the roots of the quadratic equation using the discriminant.
    ///
    /// This method uses the discriminant to determine the nature of the roots of the
    /// quadratic equation and calculates the roots accordingly.
    ///
    /// # Returns
    ///
    /// A `Roots` struct representing the roots of the quadratic equation.
    fn roots(self) -> Roots<T> {
        self.discriminant().into()
    }

    /// Calculates the sum of roots using Vieta's Formulas.
    ///
    /// The sum of roots (α + β) is calculated as `-b/a`, where:
    /// - `α` and `β` are the roots of the quadratic equation,
    /// - `a` is the coefficient of the leading term, and
    /// - `b` is the coefficient of the linear term.
    ///
    /// # Returns
    ///
    /// The sum of the roots of the quadratic equation.
    fn sum_of_roots(self) -> SumOfRoots<T>;

    /// Calculates the product of roots using Vieta's Formulas.
    ///
    /// The product of roots (α * β) is calculated as `c/a`, where:
    /// - `α` and `β` are the roots of the quadratic equation,
    /// - `a` is the coefficient of the leading term, and
    /// - `c` is the constant term.
    ///
    /// # Returns
    ///
    /// The product of the roots of the quadratic equation.
    fn product_of_roots(self) -> ProductOfRoots<T>;

    /// Calculates the axis of symmetry for the quadratic equation.
    ///
    /// The axis of symmetry is a vertical line that passes through the vertex of the parabola.
    /// It is given by the formula: `x = -b / (2a)`, where `a` and `b` are the coefficients of the
    /// quadratic equation `ax^2 + bx + c = 0`.
    ///
    /// # Returns
    ///
    /// The x-coordinate of the axis of symmetry.
    fn axis_of_symmetry(self) -> AxisOfSymmetry<T>;

    /// Determines the concavity of the parabola based on the coefficient `a`.
    ///
    /// If `a` is greater than 0, the parabola opens upward (concave upward).
    /// If `a` is less than 0, the parabola opens downward (concave downward).
    /// If `a` is equal to 0, concavity is undefined 
    fn concavity(self) -> Concavity<T>;
}