//! # Sorters
//! Implement the `Sorter` trait to create a custom Sorter or use one of the default sorters

/// Sorts the given input
pub trait Sorter {
    /// Sort
    fn sort(&self);
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
