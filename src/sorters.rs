//! # Sorters
//! Implement the `Sorter` trait to create a custom Sorter or use one of the default sorters

use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::fs::DirEntry;

/// Sorts the given input
pub trait Sorter<T> {
    /// Sort
    fn sort(&self, buffer: &mut Vec<T>) -> Result<()>;
}

impl<T> Debug for dyn Sorter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sorter")
    }
}

/// The `TimeSorter` sorts the entries by their creation time, newest first
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct TimeSorter;

impl TimeSorter {
    /// Create a new `TimeSorter`
    pub fn new() -> Self {
        Default::default()
    }
}

impl Sorter<DirEntry> for TimeSorter {
    fn sort(&self, buffer: &mut Vec<DirEntry>) -> Result<()> {
        buffer.sort_unstable_by(|left, right| {
            left.metadata()
                .unwrap()
                .created()
                .unwrap()
                .cmp(&right.metadata().unwrap().created().unwrap())
        });

        Ok(())
    }
}
