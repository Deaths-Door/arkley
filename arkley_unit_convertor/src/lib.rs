#![doc = include_str!("../README.md")]

#![forbid(
    missing_docs,
    missing_debug_implementations,

    unsafe_code,
    unused_variables,
    unused_mut,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,

    unstable_features,
    unsafe_op_in_unsafe_fn,

    trivial_casts,
    overflowing_literals,
    non_snake_case
)]

use strum::*;


pub(crate) static TIME_UNITS_TABLE : [f64;TimeUnits::COUNT - 1] = [
    100.0,      // Conversion from centuries to decades
    10.0,       // Conversion from decades to years
    12.0,       // Conversion from years to months
    4.34524,    // Conversion from months to weeks (average)
    7.0,        // Conversion from weeks to days
    24.0,       // Conversion from days to hours
    60.0,       // Conversion from hours to minutes
    60.0,       // Conversion from minutes to seconds
    1000.0,     // Conversion from seconds to milliseconds
    1000.0,     // Conversion from milliseconds to microseconds
];

macro_rules! generate_convertor {
    ($name : ident => $units : ident => { $( $variant:ident($($value:expr),* => $short : expr => $fn : ident) ),* } => $table : ident => $array_type : ty) => {
        #[doc = concat!(" A utility providing a convenient way to convert from and to `",stringify!($units),"`")]
        pub struct $name <N>{
            number : N ,
            current_unit : $units
        }

        impl<N : std::fmt::Display> std::fmt::Display for $name <N>{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f,"{} {}",self.number,self.current_unit)
            }
        }

        impl<N : std::fmt::Debug> std::fmt::Debug for $name <N> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f,"{:?} {:?}",self.number,self.current_unit)
            }
        }

        impl<N> $name <N> {
            #[doc = concat!("Creates new instance of ", stringify!($name), " from given number and unit")]
            pub const fn new(number : N,current_unit : $units) -> Self {
                Self { number , current_unit }
            } 

            #[doc = concat!(
                "Returns a reference to the numeric value stored in the ",stringify!($name),
                ". This method allows you to access the numeric value without consuming the ",stringify!($name),
                ". The reference is read-only and provides access to the stored numeric value."  
            )]
            /// # Returns
            ///
            /// A reference to the numeric value.
            pub const fn number(&self) -> &N {
                &self.number
            }


            #[doc = concat!(
                "Returns a reference to the current unit of ",stringify!($name),
                ". This method allows you to access the current unit without consuming the",stringify!($name),
                ". The reference is read-only and provides access to the stored unit."  
            )]
            /// # Returns
            ///
            /// A reference to the unite.
            pub const fn current_unit(&self) -> & $units {
                &self.current_unit
            }

        }

        impl<N> $name <N> where N : Into<$array_type> + std::ops::MulAssign + std::ops::DivAssign , f64 : Into<N> {
            /// Converts the current value to the specified unit.
            ///
            /// This method takes the current value and converts it to the specified `to_unit`.
            /// The conversion is performed in place, and the method returns the updated converter.
            pub fn convert_to(mut self,to_unit : $units) -> Self {
                if self.current_unit == to_unit {
                    return self;
                }

                let mut index = self.current_unit as usize;
                let target = to_unit as usize;
                
                match index < target {
                    true => while index < target{
                        self.number *= ($table [index]).into();
                        index += 1;
                    },
                    false => while index > target {
                        self.number /= ($table [index - 1]).into();
                        index -= 1;
                    }
                };

                self.current_unit = to_unit;
                self
            }

            $(
                #[doc = concat!(" A utility providing a convenient way to convert from the current unit to `",stringify!($variant),"`")]
                pub fn $fn (self) -> Self {
                    self.convert_to(<$units>::$variant)
                } 
            )*
        }

        #[doc = concat!("Enum representing the units of ", stringify!($name))]
        #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, EnumString, FromRepr, AsRefStr, IntoStaticStr, EnumVariantNames, EnumCount, EnumIter)]
        pub enum $units {
            $(
                #[doc = concat!("Represents the ", stringify!($variant), " unit")]
                #[strum($(serialize = $value ,)*)]
                $variant,
            )*
        }

        impl std::fmt::Display for $units {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        $units :: $variant => write!(f,"{}",$short),
                    )*
                }
            }
        }
    }
}

generate_convertor!(Time => TimeUnits => {
    Centuries("century","centuries" => "centuries" => to_centuries),
    Decades("decade","decades" => "decades"  => to_decades),
    Years("year","years","yr" => "years" => to_years),
    Months("month","months" => "months" => to_months),
    Weeks("week","weeks" => "weeks" => to_weeks),
    Days("day","days" => "days" => to_days),
    Seconds("second","seconds","s" => "s" => to_seconds),
    Hours("hour","hours","hr" => "hr" => to_hours),
    Minutes("minute","minutes","min" => "min" => to_minutes),
    Milliseconds("millisecond" ,"milliseconds" ,"ms" => "ms" => to_milliseconds),
    Microseconds("microsecond" ,"microseconds" ,"μs" => "μs" => to_microseconds)
} => TIME_UNITS_TABLE => f64);

/*=> TIME_UNITS_TABLE => f64 =>
    to_centuries and TimeUnits::Centuries
*/
/*
generate_units_enum!(TimeUnits =>
    
);*/

