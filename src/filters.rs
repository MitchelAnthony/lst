//! # Filters
//! Implement the `Filter` trait to create a custom Filter or use one of the default filters

use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::fs::DirEntry;

/// Filters the input
pub trait Filter<T> {
    /// Filter
    fn filter(&self, buffer: &mut Vec<T>) -> Result<()>;
}

impl<T> Debug for dyn Filter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Filter")
    }
}

/// The `DotFilesFilter` filters out the entries that start with a dot (.)
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct DotFilesFilter;

impl DotFilesFilter {
    /// Create a new `DotFilesFilter`
    pub fn new() -> Self {
        Default::default()
    }
}

impl Filter<DirEntry> for DotFilesFilter {
    fn filter(&self, buffer: &mut Vec<DirEntry>) -> Result<()> {
        let mut index = 0;
        while index < buffer.len() {
            if buffer[index].file_name().to_string_lossy().starts_with('.') {
                // Use swap_remove for better performance in large directories.
                // The entries are unsorted at this time and will be sorted in the next processing step.
                buffer.swap_remove(index);
            } else {
                index += 1;
            }
        }

        Ok(())
    }
}
