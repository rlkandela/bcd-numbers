mod error;
mod bcd;
//mod bcd2;

pub use error::BCDConversionError;
//pub use bcd::{FromBCD, ToBCD};
pub use bcd::{BCD, Convertible};
