//! # Validators
//! Implement the `Validator` trait to create a custom Validator or use one of the default validators

use anyhow::{bail, Result};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// Validates the given location `ls` is going to be using
pub trait Validator {
    /// Validate the `location`
    fn validate(&self, location: &str) -> Result<()>;
}

/// The error type for errors during validation
#[derive(Copy, Clone, Debug)]
pub struct ValidationError;

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No such file or directory")
    }
}

impl Error for ValidationError {}

/// The `FileSystemValidator` validates that the file or directory exists on the filesystem
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct FileSystemValidator;

impl FileSystemValidator {
    /// Create a new `FileSystemValidator`
    pub fn new() -> Self {
        Default::default()
    }
}

impl Validator for FileSystemValidator {
    fn validate(&self, location: &str) -> Result<()> {
        if !Path::new(location).exists() {
            bail!(ValidationError);
        }

        Ok(())
    }
}
