//! # Readers
//! Implement the `Reader` trait to create a custom Reader or use one of the default readers

use anyhow::{Context, Result};
use std::fs::DirEntry;
use std::path::Path;

/// Reads from the given location
pub trait Reader<T> {
    /// Read from `location` and put the resulting `T`'s in the `buffer`
    fn read(&self, location: &str, buffer: &mut Vec<T>) -> Result<()>;
}

/// The `FileSystemReader` reads the file or directory from the filesystem
///
/// # Examples
/// ```
/// # use anyhow::Result;
/// # use lst::Reader;
/// # use lst::readers::FileSystemReader;
/// # fn main() -> Result<()> {
/// #
/// let mut buffer = vec![];
/// let reader = FileSystemReader::new();
/// reader.read("./", &mut buffer)?;
/// #
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct FileSystemReader;

impl FileSystemReader {
    /// Create a new `FileSystemReader`
    pub fn new() -> Self {
        Default::default()
    }
}

impl Reader<DirEntry> for FileSystemReader {
    fn read(&self, location: &str, buffer: &mut Vec<DirEntry>) -> Result<()> {
        // TODO Continue to use DirEntry or create new type to clean up the file read part?
        let path = Path::new(location);
        if path.is_dir() {
            for entry in path
                .read_dir()
                .context("Something went wrong while reading from the directory")?
                .flatten()
            {
                buffer.push(entry);
            }
        } else if path.is_file() {
            if let Some(parent) = path.parent() {
                for entry in parent
                    .read_dir()
                    .context("Something went wrong while reading from the directory")?
                    .flatten()
                {
                    if entry.path().eq(path) {
                        buffer.push(entry);

                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
