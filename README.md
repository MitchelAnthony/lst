# LST
Rust implementation of the POSIX `ls` command.

[![Crates.io](https://img.shields.io/crates/v/lst.svg)](https://crates.io/crates/lst)
[![Documentation](https://docs.rs/lst/badge.svg)](https://docs.rs/lst/)
[![Dependency status](https://deps.rs/repo/github/MitchelAnthony/lst/status.svg)](https://deps.rs/repo/github/MitchelAnthony/lst)

## How to use

This example will print a string with all non-hidden file and directory names, sorted by creation time.
```rust
use anyhow::Result;
use lst::filters::DotFilesFilter;
use lst::formatters::NameOnlyFormatter;
use lst::readers::FileSystemReader;
use lst::sorters::TimeSorter;
use lst::validators::FileSystemValidator;
use lst::{Location, Lst};

fn main() -> Result<()> {
  let mut lst = Lst::new(Location::new("./"));
  lst.validator(FileSystemValidator::new())
     .reader(FileSystemReader::new())
     .filter(Box::new(DotFilesFilter::new()))
     .sorter(Box::new(TimeSorter::new()))
     .formatter(NameOnlyFormatter::new());
  
  println!("{}", lst.generate()?);
  
  Ok(())
}
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, 
shall be dual licensed as above, without any additional terms or conditions.
