//! read and write

use crate::{Error, Meta};
use std::{
    fs,
    fs::File,
    io::{BufWriter, Read as StdRead, Write as StdWrite},
    path::PathBuf,
};

/// io read
pub trait Read<'r>: Meta<'r> {
    /// read stream from file
    fn read(&'r self) -> Result<Vec<u8>, Error> {
        let mut src = PathBuf::from(self.base()?);
        src.push(self.name()?);

        let mut f = File::open(src)?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

/// io write
pub trait Write<'w>: Meta<'w> {
    /// write stream into file
    fn write<B>(&'w self, stream: B) -> Result<(), Error>
    where
        B: AsRef<[u8]>,
    {
        let mut src = PathBuf::from(self.base()?);
        src.push(self.name()?);

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

impl<'w, T> Read<'w> for T where T: Meta<'w> {}
impl<'w, T> Write<'w> for T where T: Meta<'w> {}
