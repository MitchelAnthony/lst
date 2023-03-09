#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]

//! Rust implementation of the POSIX `ls` command

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_dir;
use std::path::Path;

/// This is our main entry point
/// # Examples
/// ```
/// # use lst::ls;
/// ls();
/// ```
pub fn ls() {
    let path = Path::new("./");
    debug_assert!(path.is_dir());

    let dir_contents = read_dir(path).unwrap();
    for entry in dir_contents {
        println!("{}", entry.unwrap().file_name().into_string().unwrap());
    }
}

#[derive(Debug)]
struct LstError;

impl Display for LstError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Error for LstError {}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
