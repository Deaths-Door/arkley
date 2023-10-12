/*/*mod solution;
mod interger;

pub use interger::*;

use self::solution::Solution;*/

pub trait Quadratic<T> : std::fmt::Display + std::fmt::Debug + Clone {
    fn using_abc(self) -> ShreedharaAcharya;
   // fn using_pq(self) -> Solution<T>;
}*/