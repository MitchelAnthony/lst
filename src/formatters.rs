//! # Formatters
//! Implement the `Formatter` trait to create a custom Formatter or use one of the default formatters

use anyhow::Result;
use std::fmt::Write;
use std::fs::DirEntry;

/// Formats the output
pub trait Formatter<T> {
    /// Format
    fn format(&self, buffer: &[T]) -> Result<String>;
}

/// The `NameOnlyFormatter` ... TODO
///
/// # Examples
/// ```
/// # use anyhow::Result;
/// # use lst::Formatter;
/// # use lst::formatters::NameOnlyFormatter;
/// # fn main() -> Result<()> {
/// #
/// let buffer = vec![];
/// let formatter = NameOnlyFormatter::new();
/// formatter.format(&buffer)?;
/// #
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct NameOnlyFormatter;

impl NameOnlyFormatter {
    /// Create a new `NameOnlyFormatter`
    pub fn new() -> Self {
        Default::default()
    }
}

impl Formatter<DirEntry> for NameOnlyFormatter {
    fn format(&self, buffer: &[DirEntry]) -> Result<String> {
        let mut string_buffer = String::new();
        for direntry in buffer {
            writeln!(
                &mut string_buffer,
                "{}",
                // TODO Check if this conversion might cause issues
                direntry.file_name().to_string_lossy()
            )?;
        }

        Ok(string_buffer)
    }
}
