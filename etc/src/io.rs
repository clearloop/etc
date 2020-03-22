//! read and write

use crate::{Error, Meta};
use std::{
    fs::File,
    io::{BufWriter, Read as StdRead, Write as StdWrite},
    path::PathBuf,
};

/// io read
pub trait Read<'r>: Meta<'r> {
    /// read stream from file
    fn read(&'r self) -> Result<Vec<u8>, Error> {
        let mut src = PathBuf::from(self.base());
        src.push(self.path());

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
        B: AsRef<&'w [u8]>,
    {
        let mut src = PathBuf::from(self.base());
        src.push(self.path());

        if !src.exists() {
            File::create(&src)?;
        }

        let f = File::open(src)?;
        let mut writer = BufWriter::new(f);

        writer.write(stream.as_ref())?;
        Ok(())
    }
}
