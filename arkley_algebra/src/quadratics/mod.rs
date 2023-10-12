mod interger;
mod discriminant;
mod roots;

pub use interger::*;
pub use discriminant::*;
pub use roots::*;


/// A trait for working with quadratic equations and their properties.
///
/// The `Quadratic` trait provides a set of methods for working with quadratic
/// equations, including calculating the discriminant and determining the nature
/// of roots. It allows for easy extension with custom types.
pub trait Quadratic<T> : Sized {
    /// Calculates the discriminant of the quadratic equation.
    ///
    /// The discriminant of a quadratic equation `ax^2 + bx + c` is given by `D = b^2 - 4ac`.
    ///
    /// # Returns
    ///
    /// The discriminant value of the quadratic equation.
    fn discriminant(self) -> Discriminant<Self>;

    /// Calculates the roots of the quadratic equation using the discriminant.
    ///
    /// This method uses the discriminant to determine the nature of the roots of the
    /// quadratic equation and calculates the roots accordingly.
    ///
    /// # Returns
    ///
    /// A `Roots` struct representing the roots of the quadratic equation.
    fn roots(self) -> Roots<Self> {
        self.discriminant().into()
    }
}