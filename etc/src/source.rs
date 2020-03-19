//! etc source

use std::collections::HashMap;

/// contains dir and file
pub struct Source<'s> {
    /// source name
    pub name: &'s str,

    /// source tree
    pub tree: HashMap<&'s str, Box<Source<'s>>>,

    /// source type
    pub srcty: EtcSource,

    /// source stream
    pub stream: &'s [u8],
}

/// etc source enum
pub enum EtcSource {
    /// etc dir
    Dir,
    /// etc file
    File,
}
