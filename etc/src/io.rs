//! read and write

use crate::{Error, Meta};
use std::{
    fs,
    fs::File,
    io::{BufWriter, Read as StdRead, Write as StdWrite},
};

/// io read
pub trait Read: Meta {
    /// read stream from file
    fn read(&self) -> Result<Vec<u8>, Error> {
        let mut f = File::open(self.real_path()?)?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

/// io write
pub trait Write: Meta {
    /// write stream into file
    fn write<B>(&self, stream: B) -> Result<(), Error>
    where
        B: AsRef<[u8]>,
    {
        let src = self.real_path()?;
        if let Some(parent) = src.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        } else {
            return Err(Error::Custom(format!(
                "Invalid file path: {}",
                self.real_path()?.to_string_lossy(),
            )));
        }

        let f = File::create(src)?;
        let mut writer = BufWriter::new(f);

        writer.write_all(stream.as_ref())?;
        writer.flush()?;
        Ok(())
    }
}

impl<T> Read for T where T: Meta {}
impl<T> Write for T where T: Meta {}
