# voyager-clients (nerva)

> [![version-shield]][crate-link] [![downloads-shield]][crate-link] [![docs-build-shield]][docs-url] [![contributors-shield]][contributors-url] [![license-shield]][license-url] [![issues-shield]][issues-url]

## Overview

_voyager-clients (nerva)_ is a crate that contains client implementations for [NASA's Open APIs](https://api.nasa.gov). The crate and codebase go by the codename
_voyager-clients_, while the library itself goes by the name **nerva**. Documentation for _nerva_ can be found [here](https://docs.rs/voyager_client/0.3.4/voyager_client/). For examples, or information on contributing, please read below.

## Usage

```Rust
use nerva::prelude::*;
use nerva::client::apod::*;
use nerva::core::Filter;
use nerva::filters::{filter, Match};

fn main() where
{
    let apod = Apod::default();
    // Query APOD for date = 2023-02-21
    let response = apod.get(ApodParams::date("2023-02-21"));
    // Filter for the "explanation" key
    let values = filter::<Match<String>>(response.unwrap(), &Match::new("explanation"));
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
