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

        impl<N> $name <N> where N : From<$array_type> + std::ops::MulAssign + std::ops::DivAssign {
            /// Converts the current value to the specified unit.
            ///
            /// This method takes the current value and converts it to the specified `to_unit`.
            /// The conversion is performed in place, and the method returns the updated converter.
            pub fn convert_to(mut self, to_unit: $units) -> Self {
                if self.current_unit == to_unit {
                    println!("{} is same as {to_unit}",self.current_unit);
                    return self;
                }
            
                let index_from = self.current_unit as usize;
                let index_to = to_unit as usize;

                println!("{index_from}..{index_to}");
            
                match index_from < index_to {
                    true => for index in index_from..index_to {
                        println!("/= {}",$table[index]);
                        self.number /= $table[index].into();
                    },
                    false => for index in (index_to..index_from) {
                        println!("*= {}",$table[index - 1]);
                        self.number *= $table[index].into();
                    },
                }
            
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
    Microseconds("microsecond" ,"microseconds" ,"μs" => "μs" => to_microseconds),
    Milliseconds("millisecond" ,"milliseconds" ,"ms" => "ms" => to_milliseconds),
    Seconds("second","seconds","s" => "s" => to_seconds),
    Minutes("minute","minutes","min" => "min" => to_minutes),
    Hours("hour","hours","hr" => "hr" => to_hours),
    Days("day","days" => "days" => to_days),
    Weeks("week","weeks" => "weeks" => to_weeks),
    Months("month","months" => "months" => to_months),
    Years("year","years","yr" => "years" => to_years),
    Decades("decade","decades" => "decades"  => to_decades),
    Centuries("century","centuries" => "centuries" => to_centuries)
} => TIME_TABLE => f64);

static DATA_STORAGE_TABLE: [i32; DataStorageUnits::COUNT - 1] = [
    8, 
    1024,
    1024,
    1024,
    1024,
    1024,
    1024,
    1024,
    1024,
];

generate_convertor!(DataStorage => DataStorageUnits => {
    Bits("bit", "bits" => "b" => to_bits),
    Bytes("byte", "bytes" => "B" => to_bytes),
    Kilobytes("kilobyte", "kilobytes" => "KB" => to_kilobytes),
    Megabytes("megabyte", "megabytes" => "MB" => to_megabytes),
    Gigabits("gigabit", "gigabits" => "Gb" => to_gigabits),
    Terabytes("terabyte", "terabytes" => "TB" => to_terabytes),
    Petabytes("petabyte", "petabytes" => "PB" => to_petabytes),
    Exabytes("exabyte", "exabytes" => "EB" => to_exabytes),
    Zettabytes("zettabyte", "zettabytes" => "ZB" => to_zettabytes),
    Yottabytes("yottabyte", "yottabytes" => "YB" => to_yottabytes)
} => DATA_STORAGE_TABLE => i64);