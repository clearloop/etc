//! # etc
//!
//! [![etc](https://github.com/clearloop/etc/workflows/etc/badge.svg)](https://github.com/clearloop/etc)
//! [![crate](https://img.shields.io/crates/v/etc.svg)](https://crates.io/crates/etc)
//! [![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/etc/)
//! [![dependency status](https://deps.rs/repo/github/clearloop/etc/status.svg)](https://deps.rs/repo/github/clearloop/etc)
//! [![downloads](https://img.shields.io/crates/d/etc.svg)](https://crates.io/crates/etc)
//! [![LICENSE](https://img.shields.io/crates/l/etc.svg)](https://choosealicense.com/licenses/mit/)
//!
//! It's time to bundle etc for your awesome project!
//!
//! ```rust
//! use etc::{Etc, FileSystem, Read, Write};
//!
//! fn main() {
//!     // config root path
//!     let mut dir = dirs::home_dir().unwrap();
//!     dir.push(".etc.io");
//!
//!     // generate ~/.etc.io dir
//!     let etc = Etc::new(&dir).unwrap();
//!     let hello = etc.open("hello.md").unwrap();
//!
//!     // input and output
//!     assert!(hello.write(b"hello, world!\n").is_ok());
//!     assert_eq!(hello.read().unwrap(), b"hello, world!\n");
//!
//!     // remove hello.md
//!     assert!(etc.rm("hello.md").is_ok());
//!
//!     // hello.md has been removed
//!     let mut hello_md = dir.clone();
//!     hello_md.push("hello.md");
//!     assert!(!hello_md.exists());
//!
//!     // remove all
//!     assert!(etc.drain().is_ok());
//!     assert!(!dir.exists());
//! }
//! ```
//!
//! ## LICENSE
//!
//! MIT
#![warn(missing_docs)]

mod error;
mod etc;
mod fs;
mod io;
mod meta;
mod source;
mod support;

pub use crate::{
    error::Error,
    etc::Etc,
    fs::FileSystem,
    io::{Read, Write},
    meta::Meta,
    source::Source,
};

// #[cfg(feature = "derive")]
// pub use etc_derive::*;

#[cfg(feature = "bonus")]
pub use dirs;
