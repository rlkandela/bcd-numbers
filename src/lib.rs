mod error;
mod bcd;
//mod bcd2;

#[cfg(feature = "alloc")]
extern crate alloc;

pub use error::BCDConversionError;
//pub use bcd::{FromBCD, ToBCD};
pub use bcd::{BCD, Convertible, DynBCD};
