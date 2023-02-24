# voyager-clients (nerva)

> [![version-shield]][crate-link] [![downloads-shield]][crate-link] [![docs-build-shield]][docs-url] [![contributors-shield]][contributors-url] [![license-shield]][license-url] [![issues-shield]][issues-url]

## Overview

_voyager-clients (nerva)_ is a crate that contains client implementations for [NASA's Open APIs](https://api.nasa.gov). The crate and codebase go by the codename
_voyager-clients_, while the library itself goes by the name **nerva**. Documentation for _nerva_ can be found [here](https://docs.rs/voyager_client/0.3.4/voyager_client/). For examples, or information on contributing, please read below.

## Usage

```Rust
use nerva::prelude::*;
use nerva::core::Filter;
use nerva::filters::{filter, Match};

fn main() where
{
    // ---- Get the explanation of an APOD entry ------
    use nerva::clients::apod::*;
    // Get a client
    let apod = Apod::default();
    // use a default parameter
    let response = apod.get(ApodParams::default()).unwrap();
    // Filter for the "explanation" key
    let values = filter::<Match<String>>(response, &Match::new("explanation"));
    assert!(values.is_ok());
    println!("{:#?}", values.unwrap());

    // ---- Get the active region numbers from DONKI Flr ----
    use nerva::clients::donki::flr::*;
    // Get a client
    let flr = FLR::default();
    // Use a custom start date
    let params = FLRParams::StartDate("2023-01-01");
    let response = flr.get(params).unwrap();
    // Filter for the active region numbers
    let values = filter::<Match<u32>>(response, &Match::new("activeRegionNum"));
    assert!(values.is_ok());
    println!("{:#?}", values.unwrap());
}
```

[version-shield]: https://img.shields.io/crates/v/voyager_client?style=plastic
[contributors-shield]: https://img.shields.io/github/contributors/ethgallucci/voyager?style=plastic
[contributors-url]: https://github.com/ethgallucci/voyager/graphs/contributors
[issues-shield]: https://img.shields.io/github/issues/ethgallucci/voyager?style=plastic
[issues-url]: https://github.com/ethgallucci/voyager/issues
[license-shield]: https://img.shields.io/crates/l/voyager_client?style=plastic
[license-url]: https://github.com/ethgallucci/voyager/blob/main/LICENSE
[commit-shield]: https://img.shields.io/github/commit-activity/w/ethgallucci/voyager?style=plastic
[commit-url]: https://github.com/ethgallucci/voyager/commits/main
[downloads-shield]: https://img.shields.io/crates/d/voyager_client?style=plastic
[crate-link]: https://crates.io/crates/voyager_client
[docs-build-shield]: https://img.shields.io/docsrs/voyager_client/latest?label=build&style=plastic
[docs-url]: https://docs.rs/voyager_client
