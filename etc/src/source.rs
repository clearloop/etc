//! etc source

use std::collections::HashMap;

/// contains dir and file
#[derive(Clone, Default)]
pub struct Source<'s> {
    /// source namep
    pub name: &'s str,

    /// source tree
    pub tree: HashMap<&'s str, Box<Source<'s>>>,

    /// source type
    pub ty: EtcSource,

    /// source stream
    pub stream: &'s [u8],
}

/// etc source enum
#[derive(Clone)]
pub enum EtcSource {
    /// etc dir
    Dir,
    /// etc file
    File,
}

impl Default for EtcSource {
    fn default() -> EtcSource {
        EtcSource::File
    }
}

impl<'s> Source<'s> {
    /// generate etc dir
    pub fn dir(name: &'s str) -> Source<'s> {
        Source {
            name,
            stream: &[],
            tree: HashMap::new(),
            ty: EtcSource::Dir,
        }
    }

    /// generate etc file
    pub fn file<S>(name: &'s str, stream: S) -> Source<'s>
    where
        S: AsRef<&'s [u8]>,
    {
        Source {
            name,
            stream: stream.as_ref(),
            tree: HashMap::new(),
            ty: EtcSource::File,
        }
    }
}
