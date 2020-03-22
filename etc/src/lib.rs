//! etc
#![warn(missing_docs)]

mod error;
mod etc;
mod fs;
mod io;
mod meta;
mod source;
mod support;

pub use crate::{error::Error, etc::Etc, fs::FileSystem, meta::Meta, source::Source};

// #[cfg(feature = "derive")]
// pub use etc_derive::*;

#[cfg(feature = "bonus")]
pub use dirs;
