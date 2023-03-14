//! Rust implementation of the POSIX `ls` command

#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]

pub mod filters;
pub mod formatters;
pub mod readers;
pub mod sorters;
pub mod validators;

pub use filters::Filter;
pub use formatters::Formatter;
pub use readers::Reader;
pub use sorters::Sorter;
pub use validators::Validator;

use crate::filters::DotFilesFilter;
use crate::formatters::NameOnlyFormatter;
use crate::readers::FileSystemReader;
use crate::sorters::TimeSorter;
use crate::validators::FileSystemValidator;

/// TODO Docs
#[non_exhaustive]
// #[derive(Debug)] // TODO Make sure this'll work again. Maybe add copy?
pub struct Lst<T, U, V>
where
    T: Formatter,
    U: Reader,
    V: Validator,
{
    location: Location,
    // Should these all use dynamic dispatch and support multiple processors?
    filters: Vec<Box<dyn Filter>>,
    formatter: T,
    reader: U,
    sorters: Vec<Box<dyn Sorter>>,
    validator: V,
}

impl Lst<NameOnlyFormatter, FileSystemReader, FileSystemValidator> {
    /// Creates a new `Lst`
    pub fn new(location: Location) -> Self {
        Lst {
            location,
            filters: vec![Box::new(DotFilesFilter::new())],
            formatter: NameOnlyFormatter::new(),
            reader: FileSystemReader::new(),
            sorters: vec![Box::new(TimeSorter::new())],
            validator: FileSystemValidator::new(),
        }
    }
}

impl<T, U, V> Lst<T, U, V>
where
    T: Formatter,
    U: Reader,
    V: Validator,
{
    /// Set the `Filter` for `Lst` to use
    pub fn filter(&mut self, filter: Box<dyn Filter>) -> &mut Self {
        self.filters.clear(); // For now, we only support one Filter
        self.filters.push(filter);

        self
    }

    /// Set the `Formatter` for `Lst` to use
    pub fn formatter(&mut self, formatter: T) -> &mut Self {
        self.formatter = formatter;

        self
    }

    /// Set the `Reader` for `Lst` to use
    pub fn reader(&mut self, reader: U) -> &mut Self {
        self.reader = reader;

        self
    }

    /// Set the `Sorter` for `Lst` to use
    pub fn sorter(&mut self, sorter: Box<dyn Sorter>) -> &mut Self {
        self.sorters.clear(); // For now, we only support one Sorter
        self.sorters.push(sorter);

        self
    }

    /// Set the `Validator` for `Lst` to use
    pub fn validator(&mut self, validator: V) -> &mut Self {
        self.validator = validator;

        self
    }

    /// Generate the output TODO better docs
    pub fn generate(&self) {
        println!("Generating...");

        println!("{:?}", self.location);
        self.validator.validate();
        self.reader.read();
        for filter in self.filters.iter() {
            filter.filter();
        }
        for sorter in self.sorters.iter() {
            sorter.sort();
        }
        self.formatter.format();

        println!("Done!")
    }
}

/// The files and directories from will be read from this location. It's not a Path and just wraps a
/// (owned) string to allow the end user all flexibility when implementing custom processors.
///
/// # Examples
/// ```
/// # use lst::Location;
/// let string = String::from("./");
/// let location_1 = Location::new(&string);
/// let location_2 = Location::new(string);
/// let location_3 = Location::new("./");
/// ```
#[derive(Clone, Debug, Default)]
pub struct Location(String);

impl Location {
    /// Creates a new `Location` using any type of string (reference)
    pub fn new<T: AsRef<str>>(location: T) -> Self {
        println!("{}", location.as_ref());

        Location(location.as_ref().to_string())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
