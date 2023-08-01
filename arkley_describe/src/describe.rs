use crate::{FilterLevel,Step, SubStep};

/// Represents a generic trait for describing operations.
/// The associated type `Output` specifies the return type of the `describe` method.
pub trait Describe<Rhs = Self>: Sized {
    /// The output type returned by the `describe` method (with is [crate::Step] for arkley).
    type Output;

    /// Describes the addition operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the addition.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the addition operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_add(self,other : Rhs,filter_level : FilterLevel) -> Option<Self::Output>;

    // Describes the subtraction operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the subtraction.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the subtraction operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_sub(self, other: Rhs, filter_level: FilterLevel) -> Option<Self::Output>;

    /// Describes the multiplication operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the multiplication.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the multiplication operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_mul(self, other: Rhs, filter_level: FilterLevel) -> Option<Self::Output>;

    /// Describes the division operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the division.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the division operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_div(self, other: Rhs, filter_level: FilterLevel) -> Option<Self::Output>;
}


impl Describe<f64> for f64 {
    type Output = Step;

    fn describe_add(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        todo!()
    }

    fn describe_sub(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        todo!()
    }

    fn describe_mul(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        const BASE : u32 = 10;
        const TITLE : &str = "Column Multiplication";
        const DESCRIPTION : &str =  "Multiply each digit in the second number with the digits in the first number, and write the results below each digit in the second number.";

        let mut step = Step::new(TITLE.to_string(),DESCRIPTION.to_string());

        if self.is_negative() || other.is_negative() {
            step.insert_to_description("\nSince we have a negative number lets ignore the sign for now.");
        }

        let (x,y) = swap_if_greater(self,other);

        let (padding,longest_decimal)  = figure_alignment(x,y);

        let c_aligned = into_column(x,y,"+",padding,longest_decimal);

        let x_space_index = c_aligned[0].rfind(' ').unwrap();
        let y_space_index = c_aligned[0].rfind(' ').unwrap();

        // So only valid numbers are there in loop
        let x_str = &c_aligned[0][x_space_index + 1..];
        let y_str = &c_aligned[1][y_space_index + 1..];

        // to take into account decimal points in numbers for the factor scaling so 10 to the power of index - (encounter as i32)
        let mut x_dec_encounted = false;
        let mut y_dec_encounted = false;

        for  (y_index,y_ch) in y_str.chars().rev().enumerate() {
            if y_ch == '.' {
                y_dec_encounted = true;
                continue;
            }

            let yd = y_ch.to_digit(BASE).unwrap_or(0);

            if yd == 0 {
                if y_index == y_str.len() - 1 {
                    continue;    
                }
                
                let format = format!("Now we can skip multiplying {x_str} with 0 as {x_str} * 0 = 0");
                let substep = SubStep::new(format);
                step.add_substep(substep);
                continue;
            }
        }

        todo!("ADD DIAGRAMS TO IT IN THE FIRST PLACE LIKE THE WHOLE POINT OF IT")
    }

    fn describe_div(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        todo!()
    }
}

fn swap_if_greater(x: f64, y: f64) -> (f64, f64) {
    match x > y {
        true => (x, y),
        false => (y, x)
    } 
}

fn align(number : f64,padding : usize,longest_decimal : usize) -> String {
    format!("{:width$.dec$}",number,width = padding,dec = longest_decimal)
}

fn figure_alignment(x : f64,y : f64) -> (usize,usize) {
    let x_str = x.to_string();
    let y_str = y.to_string();

    let padding = x_str.len().max(y_str.len()) + 6;

    let mut longest_decimal : usize = 0 ;

    let mut closure = |string : &str|{
        let index = string.find('.').unwrap();
        let dec_len = string.len() - index - 1;

        if dec_len > longest_decimal {
            longest_decimal = dec_len
        };
    };

    
    if x.fract() != 0.0 {
        closure(&x_str);
    }

    if y.fract() != 0.0 {
        closure(&y_str);
    }

    (padding,longest_decimal)
}

fn into_column(x : f64,y : f64,op_str : &str,padding : usize,longest_decimal : usize) -> Vec<String> {
    let padded_x = format!("{:width$.dec$}", x, width = padding,dec = longest_decimal);
    let padded_y = format!("{} {:width$.dec$}",op_str,y,width = padding - 2,dec = longest_decimal);
    let seperator = "-".repeat(padding);

    return vec![padded_x,padded_y,seperator]
}