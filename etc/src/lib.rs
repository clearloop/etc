//! cfger
#![warn(missing_docs)]

mod error;
mod etc;
mod source;
mod support;

pub use crate::{
    error::Error,
    etc::Etc,
    source::{EtcSource, Source},
};

#[cfg(feature = "derive")]
pub use etc_derive::*;
