//! # Readers
//! Implement the `Reader` trait to create a custom Reader or use one of the default readers

/// Reads from the given location
pub trait Reader {
    /// Read
    fn read(&self);
}

/// TODO
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct FileSystemReader;

impl FileSystemReader {
    /// TODO
    pub fn new() -> Self {
        Default::default()
    }
}

impl Reader for FileSystemReader {
    fn read(&self) {
        println!("Reading...");
    }
}
