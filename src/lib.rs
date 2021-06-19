#![feature(min_type_alias_impl_trait)]

use std::convert::TryInto;

use bit_index::*;

mod error;
mod factorial;
mod permutation_generator;
mod single_permutation;

pub use error::*;
pub(crate) use factorial::*;
pub use permutation_generator::*;
pub(crate) use single_permutation::*;
