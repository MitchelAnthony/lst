//! # Sorters
//! Implement the `Sorter` trait to create a custom Sorter or use one of the default sorters

use std::fmt::{Debug, Formatter};

/// Sorts the given input
pub trait Sorter {
    /// Sort
    fn sort(&self);
}

impl Debug for dyn Sorter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sorter")
    }
}

/// TODO
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct TimeSorter;

impl TimeSorter {
    /// TODO
    pub fn new() -> Self {
        Default::default()
    }
}

impl Sorter for TimeSorter {
    fn sort(&self) {
        println!("Sorting...");
    }
}
