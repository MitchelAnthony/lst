//! # Filters
//! Implement the `Filter` trait to create a custom Filter or use one of the default filters

use std::fmt::{Debug, Formatter};

/// Filters the input
pub trait Filter {
    /// Filter
    fn filter(&self);
}

impl Debug for dyn Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Filter")
    }
}

/// TODO
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct DotFilesFilter;

impl DotFilesFilter {
    /// TODO
    pub fn new() -> Self {
        Default::default()
    }
}

impl Filter for DotFilesFilter {
    fn filter(&self) {
        println!("Filtering...");
    }
}
