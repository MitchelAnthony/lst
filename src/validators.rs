//! # Validators
//! Implement the `Validator` trait to create a custom Validator or use one of the default validators

/// Validates the given location `ls` is going to be using
pub trait Validator {
    /// Validate
    fn validate(&self);
}

/// TODO
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct FileSystemValidator;

impl FileSystemValidator {
    /// TODO
    pub fn new() -> Self {
        Default::default()
    }
}

impl Validator for FileSystemValidator {
    fn validate(&self) {
        println!("Validating...");
    }
}
