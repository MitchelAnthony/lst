//! # Formatters
//! Implement the `Formatter` trait to create a custom Formatter or use one of the default formatters

/// Formats the output
pub trait Formatter {
    /// Format
    fn format(&self);
}

/// TODO
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct NameOnlyFormatter;

impl NameOnlyFormatter {
    /// TODO
    pub fn new() -> Self {
        Default::default()
    }
}

impl Formatter for NameOnlyFormatter {
    fn format(&self) {
        println!("Formatting...");
    }
}
